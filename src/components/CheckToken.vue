<script setup lang="ts">
import { ref, inject } from "vue";
import axios from 'axios'
import emitter from './eventBus';
import { ElNotification } from 'element-plus'


const hostName = inject<string>('hostName');


const form = ref({
    token: ''
})

const isError = ref(true)

interface TokenInfo {
    token: string,
    desc: string,
    valid: boolean
}

function login(): Promise<TokenInfo> {
    console.log(form.value.token)
    console.log(isError)
    if (form.value.token.length == 0) {
        ElNotification({
            title: '请输入客户端token',
            message: 'This is an error message',
            type: 'error',
        })
        throw new Error("请输入客户端token");
    }
    emitter.emit('message', form.value.token);
    const result = getData(hostName + "/api/token/check/" + form.value.token)
    return result

}

async function getData(uri: string): Promise<TokenInfo> {
    return (await axios.get(uri)).data
}


defineExpose({
    login

});
</script>

<template>
    <div class="container">
        <h1>信息采集</h1>
        <div class="form_container">
            <el-input :class="{ 'is-error': isError }" v-model="form.token" placeholder="客户端token:" />
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