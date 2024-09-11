<template>
    <div class="container">
        <el-form-item label="请输入文件夹路径: ">
            <el-input v-model="filePath" style="width: 240px" placeholder="d:\xxxx" clearable />
        </el-form-item>

        <el-button type="primary" @click="checkFilesExistAndRead">开始监测</el-button>

        <div v-if="fileContent">

        </div>
    </div>
</template>

<script setup lang="ts">
import { readTextFile, exists } from '@tauri-apps/api/fs';
import { ref, inject } from 'vue'
import axios from 'axios'
import { ElNotification } from 'element-plus'

const clientToken = inject<string>('clientToken', '');
const hostName = inject<string>('hostName', '');


const clientTokenId = ref<string>('');
clientTokenId.value = clientToken

const fileContent = ref<string | null>(null);

const defaultFilePath = "/Users/carlwang/Downloads/spider_img/"
const filePath = ref("")
const authFile = "auth.json"
const indexFile = "index.json"
const requestFile = "request.json"

const files = [authFile, indexFile, requestFile]

interface ClientOrderRequest {
    orderId: string,
    matchId: string,
    orderPayload: string,
    loginCode: string,
    token: string,
    clientTokenId: string
}

async function checkFilesExistAndRead() {
    const folderPath=filePath.value
    if (folderPath.length == 0) {
        open4()
        return
    }


    const results = await Promise.all(files.map(file => {
        const fileExist = exists(folderPath + file)
        fileExist.then(exists => {
            console.log(`file ${file} exist ${exists}`)
        })
        return fileExist
    }));

    const allExist = results.every(exists => exists);

    if (allExist) {
        console.log('All files exist.');
        const authContent = await readTextFile(folderPath + files[0]);
        const indexContent = JSON.parse(await readTextFile(folderPath + files[1]));
        const payloadText = await readTextFile(folderPath + files[2])
        const requestContent = JSON.parse(payloadText);



        const clientOrderRequest: ClientOrderRequest = {
            orderId: clientTokenId.value + "----" + requestContent.users[0].realname,
            matchId: requestContent.id,
            orderPayload: payloadText,
            loginCode: indexContent.code,
            token: authContent,
            clientTokenId: clientTokenId.value
        }
        axios.post(`${hostName}/api/order/createSimpleOrder`, clientOrderRequest)

    } else {
        console.log('One or more files do not exist.');
    }
}



const open4 = () => {
    ElNotification({
        title: '请输入文件路径',
        message: '',
        type: 'error',
    })
}
</script>

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