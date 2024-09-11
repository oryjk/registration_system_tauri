<script setup lang="ts">
import { ref, inject } from "vue";
import axios from 'axios'

import { ElNotification } from 'element-plus'
import { useRouter } from 'vue-router';


const hostName = inject<string>('hostName');


const form = ref({
    token: ''
})

const isError = ref(true)
const router = useRouter();

interface TokenInfo {
    token: string,
    desc: string,
    valid: boolean
}

async function login() {
    console.log(form.value.token)
    console.log(isError)
    if (form.value.token.length == 0) {
        ElNotification({
            title: '请输入客户端token',
            message: '一定要输入，否则无法开启信息采集',
            type: 'error',
        })
        return
    }

    const result = await getData(hostName + "/api/token/check/" + form.value.token)
    if (result.valid) {
        router.push({ name: 'Working', query: { inviteCode: form.value.token } });
    }
    else {
        ElNotification({
            title: 'token输入不正确',
            message: '请输入正确的token',
            type: 'error',
        })
    }


}

async function getData(uri: string): Promise<TokenInfo> {
    return (await axios.get(uri)).data
}

</script>

<template>
    <div class="container">
        <h1>信息采集</h1>
        <div class="form_container">
            <el-input :class="{ 'is-error': isError }" v-model="form.token" placeholder="客户端token:" />
            <el-button type="primary" @click="login">登录</el-button>
        </div>
    </div>
</template>

<style scoped lang="scss">
.container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;

}
</style>