<template>
    <div>
        <button @click="checkFilesExistAndRead">读取文件</button>
        <div v-if="fileContent">
            <h3>文件内容：</h3>
            <pre>{{ fileContent }}</pre>
        </div>
    </div>
</template>

<script setup lang="ts">
import { readTextFile, exists } from '@tauri-apps/api/fs';
import { ref, inject } from 'vue'
import axios from 'axios'

const clientToken = inject<string>('clientToken','');


const clientTokenId = ref<string>('');
clientTokenId.value = clientToken

const fileContent = ref<string | null>(null);

const filePath = "/Users/carlwang/Downloads/spider_img/"
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
    const results = await Promise.all(files.map(file => {
        const fileExist = exists(filePath + file)
        fileExist.then(exists => {
            console.log(`file ${file} exist ${exists}`)
        })
        return fileExist
    }));

    const allExist = results.every(exists => exists);

    if (allExist) {
        console.log('All files exist.');
        const authContent = await readTextFile(filePath + files[0]);
        const indexContent = JSON.parse(await readTextFile(filePath + files[1]));
        const payloadText = await readTextFile(filePath + files[2])
        const requestContent = JSON.parse(payloadText);



        const clientOrderRequest: ClientOrderRequest = {
            orderId: clientTokenId.value + "----" + requestContent.users[0].realname,
            matchId: requestContent.id,
            orderPayload: payloadText,
            loginCode: indexContent.code,
            token: authContent,
            clientTokenId: clientTokenId.value
        }
        axios.post('http://127.0.0.1:5678/api/order/createSimpleOrder', clientOrderRequest)

    } else {
        console.log('One or more files do not exist.');
    }
}

</script>

<style scoped>
/* 添加一些样式 */
</style>