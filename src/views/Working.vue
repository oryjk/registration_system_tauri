<template>
    <div class="container">
        <div class="content">
            <div class="formContainer">
                <el-form-item label="文件夹路径: ">
                    <el-input v-model="filePath" placeholder="F:\抢票\spider_img" clearable />
                </el-form-item>
                <div class="button-container">
                    <el-button type="primary" @click="checkUserInfo">开始监听新用户</el-button>
                </div>
            </div>
            <hr />
            <div class="formContainer">
                <el-form-item label="用户id: " label-width="100px">
                    <el-input v-model="userBindInfo.userId" placeholder="用户id" clearable />
                </el-form-item>
                <el-form-item label="抢票人员: " label-width="100px">
                    <el-input v-model="userBindInfo.users" placeholder="抢票人员" clearable />
                </el-form-item>
                <el-form-item label="加密密钥: " label-width="100px">
                    <el-input v-model="userBindInfo.encryptKey" placeholder="加密密钥" clearable />
                </el-form-item>
                <el-form-item label="过期时间: " label-width="100px">
                    <el-input v-model="userBindInfo.expireTime" placeholder="过期时间" clearable />
                </el-form-item>
                <el-form-item label="IV: " label-width="100px">
                    <el-input v-model="userBindInfo.iv" placeholder="IV" clearable />
                </el-form-item>
                <el-form-item label="版本: " label-width="100px">
                    <el-input v-model="userBindInfo.version" placeholder="版本" clearable />
                </el-form-item>
                <el-form-item label="座位区域: " label-width="100px">
                    <el-input v-model="userBindInfo.regions" placeholder="地区" clearable />
                </el-form-item>
                <div class="button-container">
                    <el-button type="primary" @click="bindUserInfo">绑定用户关键信息</el-button>
                </div>
            </div>
            <hr />
            <div class="formContainer checkbox-container">

                <div v-for="[userId, userInfos] in candidateOrderInfos" :key="userId" :value="userId">
                    <h3>{{ userId }}</h3>
                    <el-checkbox-group v-model="checkedOrderIds">
                        <el-checkbox v-for="orderInfo in userInfos" :key="orderInfo.orderId" :value="orderInfo.orderId">
                            {{ orderInfo.orderId }} {{ orderInfo.realName }}</el-checkbox>

                    </el-checkbox-group>
                </div>


                <div class="button-container">
                    <el-button type="primary" @click="getUserCandidateOrders">刷新候选订单</el-button>
                    <el-button type="primary" @click="createOrders">创建订单</el-button>
                </div>
            </div>
            <hr />
            <div class="formContainer checkbox-container">

                <div v-for="[userId, userInfos] in orderInfos" :key="userId" :value="userId">
                    <h3>{{ userId }}</h3>
                    <el-checkbox-group v-model="savedOrderIds">
                        <el-checkbox v-for="orderInfo in userInfos" :key="orderInfo.orderId" :value="orderInfo.orderId">
                            {{ orderInfo.orderId }} {{ orderInfo.realName }}</el-checkbox>

                    </el-checkbox-group>
                </div>


                <div class="button-container">
                    <el-button type="primary" @click="getUserOrders">查看已经保存的订单</el-button>
                </div>
            </div>
            <hr />

            <div class="formContainer checkbox-container">


                <el-checkbox-group v-model="prepareDeleteOrderIds">
                    <el-checkbox v-for="orderInfo in jobs" :key="orderInfo" :value="orderInfo">
                        {{ orderInfo }}
                    </el-checkbox>

                </el-checkbox-group>


                <div class="button-container">
                    <el-button type="danger" @click="deleteOrders">删除选中的订单</el-button>
                    <el-button type="primary" @click="getJobs">查看正在运行的订单</el-button>
                </div>
            </div>
            <hr />
            <div class="btn">

                <el-button v-show="sendOrderStatus" type="danger" @click="stopRead">停止监测</el-button>
                <el-button @click="goBack">返回</el-button>
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
const memberFile = "member.json"
const userInfoFiles = [authFile, indexFile, memberFile]
const sendOrderStatus = ref(false)
const jobs = ref([])


interface UserInfoRequest {
    userId: string,
    member: string,
    loginCode: string,
    token: string
}

interface UserBindInfo {
    userId: string,
    users: string,
    encryptKey: string,
    expireTime: string,
    iv: string,
    version: string,
    regions: string
}
const userBindInfo = ref<UserBindInfo>({
    userId: '',
    users: '',
    encryptKey: '',
    expireTime: '',
    iv: '',
    version: '',
    regions: ''
})

interface OrderInfo {
    orderId: string,
    userId: string,
    realName: string
}

const candidateOrderInfos = ref<Map<string, OrderInfo[]>>(new Map([
]))
const orderInfos = ref<Map<string, OrderInfo[]>>(new Map([
]))

const checkedOrderIds = ref<string[]>([]);

const prepareDeleteOrderIds = ref<string[]>([]);
const savedOrderIds = ref<string[]>([]);

async function checkUserInfo() {

    let folderPath = filePath.value
    if (folderPath.length == 0) {
        open4()
        return
    }

    sendOrderStatus.value = true

    intervalId.value = setInterval(sendUseInfo, 1000);
    jobIntervalId.value = setInterval(getJobs, 5000);
    getUserCandidateOrders()

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
            console.log(`移动文件 ${file} 去 ${newFilePath}`)
        }


    } else {
        console.log('One or more files do not exist.');
    }
}

function createOrders() {
    const orderIds = checkedOrderIds.value
    if (orderIds.length === 0) {
        console.log("没有选中任何候选订单，跳过")
        return;
    }
    axios.post(`${hostName}/ticket/order/createOrders`, orderIds)
        .then(response => {
            getJobs()
        })
}

function bindUserInfo() {
    axios.post(`${hostName}/ticket/order/bindUserInfo`, userBindInfo.value)
}

function deleteOrders() {
    const orderIds = prepareDeleteOrderIds.value
    axios.post(`${hostName}/ticket/order/deleteOrders`, orderIds).then(() => {
        getJobs()
    })

}

function getJobs() {
    const runningJobs = axios.get(`${hostName}/ticket/order/getJobs`)
    runningJobs.then(response => {
        jobs.value = response.data
    })
}

function getUserCandidateOrders() {
    axios.get(`${hostName}/ticket/order/getUserCandidateOrders`).then(response => {
        console.log(response.data)
        candidateOrderInfos.value.clear()
        Object.entries(response.data).forEach(userOrderEntry => {
            const userId = userOrderEntry[0]
            const userOrderInfos: OrderInfo[] = userOrderEntry[1] as OrderInfo[]

            candidateOrderInfos.value.set(userId, userOrderInfos)


        })

    })
}

function getUserOrders() {
    axios.get(`${hostName}/ticket/order/getUserOrders`).then(response => {
        console.log(response.data)
        orderInfos.value.clear()
        Object.entries(response.data).forEach(userOrderEntry => {
            const userId = userOrderEntry[0]
            const userOrderInfos: OrderInfo[] = userOrderEntry[1] as OrderInfo[]

            orderInfos.value.set(userId, userOrderInfos)


        })

    })
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

    .formContainer {
        display: flex;
        justify-content: center;
        flex-direction: column;
        align-items: center;
        width: 750px;

        .el-form-item {

            width: 100%;
        }

        .button-container {
            display: flex;
            width: 100%;
            justify-content: flex-end;
        }
    }

    .checkbox-container {
        display: flex;
        flex-direction: column;

        .el-checkbox-group {
            display: flex;
            // flex-direction: column;
            margin-bottom: 10px;
            flex-wrap: wrap;

            .el-checkbox {
                width: 150px;
            }
        }

    }

}
</style>