<template>
    <div class="container">
        <div class="content">
            <el-form-item label="请输入文件夹路径: ">
                <el-input v-model="filePath" style="width: 240px" placeholder="F:\抢票\spider_img" clearable />
            </el-form-item>
            <div class="btn">
                <el-button v-show="!sendOrderStatus" type="primary" @click="checkFilesExistAndRead">开始固定下单</el-button>
                <el-button v-show="!sendOrderStatus" @click="checkUserInfo">开始监测用户信息</el-button>
                <el-button v-show="sendOrderStatus" type="danger" @click="stopRead">停止监测</el-button>
                <el-button type="primary" @click="goBack">返回</el-button>
            </div>

            <div class="job-container">
                <p v-for="job in jobs" class="text item">任务 {{ job }} 正在运行</p>
            </div>
        </div>
        <div class="ticketInfo">
            监控次数： {{ count }}
            <div>
                新添加列表：<p v-for="orderItem in orderList" :key="orderItem.name" class="text item">
                    姓名: {{ orderItem.name }}, 区域：{{ orderItem.regionName }}, 提交时间：{{ orderItem.orderDateTime }}</p>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { readTextFile, exists, renameFile, createDir } from '@tauri-apps/api/fs';
import { ref, inject } from 'vue'
import axios from 'axios'
import { ElNotification } from 'element-plus'
import { useRoute, useRouter } from 'vue-router';
import { join, dirname } from '@tauri-apps/api/path'

const route = useRoute();
const router = useRouter();
const clientToken = route.query.inviteCode;

const hostName = inject<string>('hostName', '');


const clientTokenId = ref<string>('');

clientTokenId.value = (Array.isArray(clientToken) ? clientToken[0] : clientToken) as string || '';

const intervalId = ref(0)
const jobIntervalId = ref(0)

const filePath = ref("F:\\抢票\\spider_img")
const authFile = "auth.json"
const indexFile = "index.json"
const requestFile = "request.json"

const memberFile = "member.json"

const files = [authFile, indexFile, requestFile]

const userInfoFiles = [authFile, indexFile, memberFile]

const count = ref(0)
const sendOrderStatus = ref(false)
interface OrderInfo {
    name: string,
    regionName: string,
    orderDateTime: string
}
const orderList = ref<OrderInfo[]>([])

const jobs = ref([])



interface ClientOrderRequest {
    orderId: string,
    matchId: string,
    orderPayload: string,
    loginCode: string,
    token: string,
    clientTokenId: string
}

async function checkFilesExistAndRead() {

    let folderPath = filePath.value
    if (folderPath.length == 0) {
        open4()
        return
    }
    sendOrder()
    intervalId.value = setInterval(sendOrder, 1000);
    sendOrderStatus.value = true
}

async function sendOrder() {
    let folderPath = filePath.value

    if (!folderPath.endsWith('\\')) {
        folderPath = folderPath + '\\'
    }

    console.log("folderPath " + folderPath)


    const results = await Promise.all(files.map(file => {
        const fileExist = exists(folderPath + file)
        fileExist.then(exists => {
            console.log(`file: ${folderPath + file} exist: ${exists}`)
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

        const realname = requestContent.users[0].realname
        const region = requestContent.regions[0].name

        const clientOrderRequest: ClientOrderRequest = {
            orderId: clientTokenId.value + "|" + realname + "|" + region,
            matchId: requestContent.id,
            orderPayload: payloadText,
            loginCode: indexContent.code,
            token: authContent,
            clientTokenId: clientTokenId.value
        }
        axios.post(`${hostName}/api/order/createSimpleOrder`, clientOrderRequest)

        orderList.value.push({ name: realname, regionName: region, orderDateTime: formatDateTime(new Date()) })

        for (const file of files) {
            const newFilePath = await join(folderPath + clientTokenId.value + '\\' + realname + '\\' + region + '\\', file);
            const targetDir = await dirname(newFilePath);
            await createDir(targetDir, { recursive: true });
            await renameFile(folderPath + file, newFilePath);
        }


        count.value = count.value + 1
    } else {
        console.log('One or more files do not exist.');
    }
}

async function checkUserInfo() {

    let folderPath = filePath.value
    if (folderPath.length == 0) {
        open4()
        return
    }
    sendUseInfo()
    intervalId.value = setInterval(sendUseInfo, 1000);
    sendOrderStatus.value = true
    jobIntervalId.value = setInterval(getJobs, 5000);
}

async function sendUseInfo() {
    let folderPath = filePath.value
    if (!folderPath.endsWith('\\')) {
        folderPath = folderPath + '\\'
    }

    console.log("folderPath " + folderPath)




    const results = await Promise.all(userInfoFiles.map(file => {
        const fileExist = exists(folderPath + file)
        fileExist.then(exists => {
            console.log(`file: ${folderPath + file} exist: ${exists}`)
        })
        return fileExist
    }));

    const allExist = results.every(exists => exists);

    if (allExist) {
        console.log('All files exist.');
        const authContent = await readTextFile(folderPath + userInfoFiles[0]);
        const indexContent = JSON.parse(await readTextFile(folderPath + userInfoFiles[1]));
        const memberContent = JSON.parse(await readTextFile(folderPath + userInfoFiles[2]));
        const userId = memberContent.data[0].uid
        const userInfoRequest: UserInfoRequest = {
            userId: userId,
            member: memberContent.data,
            loginCode: indexContent.code,
            token: authContent
        }
        axios.post(`${hostName}/ticket/order/createUserInfo`, userInfoRequest)

        for (const file of userInfoFiles) {
            const newFilePath = await join(folderPath + userId + '\\', file);
            const targetDir = await dirname(newFilePath);
            await createDir(targetDir, { recursive: true });
            await renameFile(folderPath + file, newFilePath);
        }
        console.log('移动到已处理文件夹' + targetDir);

    } else {
        console.log('One or more files do not exist.');
    }
}

function getJobs() {
    const runningJobs = axios.get(`${hostName}/ticket/order/getJobs`)
    runningJobs.then(response => {
        jobs.value = response.data
    })
}



function formatDateTime(date: Date): string {
    const year = date.getFullYear();
    const month = (date.getMonth() + 1).toString().padStart(2, '0'); // 月份从0开始，所以需要加1
    const day = date.getDate().toString().padStart(2, '0');
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');
    const seconds = date.getSeconds().toString().padStart(2, '0');

    return `${year}-${month}-${day} ${hours}-${minutes}-${seconds}`;
}

const stopRead = () => {
    clearInterval(intervalId.value)
    clearInterval(jobIntervalId.value)
    sendOrderStatus.value = false
}



const open4 = () => {
    ElNotification({
        title: '请输入文件路径',
        message: '',
        type: 'error',
    })
}

const goBack = () => {
    router.push({ name: 'Login' });
};
</script>

<style scoped lang="scss">
.container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    /* 水平居中 */
    text-align: center;
    align-items: center;
    /* 垂直居中 */


    .content {
        width: 80%;
        /* 设置宽度为 80% */

    }

    .ticketInfo {
        width: 80%;
        height: 400px;
        overflow-y: auto;
        /* 当内容超出高度时显示垂直滚动条 */
    }

}
</style>