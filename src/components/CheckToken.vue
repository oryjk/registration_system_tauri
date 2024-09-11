<script setup lang="ts">
import { ref } from "vue";
import axios from 'axios'

const form = ref({
    token: ''
})

const hostName = "https://oryjk.cn:82"

interface TokenInfo {
    token: string,
    desc: string,
    valid: boolean
}

async function login(): Promise<TokenInfo> {
    console.log(form.value.token)
    const result = await getData(hostName + "/api/token/check/" + form.value.token)

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

            <el-form-item label="客户端token">
                <el-input v-model="form.token" />
            </el-form-item>

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