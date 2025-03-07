<script setup lang="ts">
import {createDir, exists, readTextFile, renameFile} from '@tauri-apps/api/fs'
import {inject, onMounted, Ref, ref} from 'vue'
import axios from 'axios'
import {ElNotification} from 'element-plus'
import {useRoute, useRouter} from 'vue-router';
import {dirname, join} from '@tauri-apps/api/path'

const route = useRoute()
const router = useRouter()
const clientToken = route.query.inviteCode
const hostName = inject<Ref<string>>('hostName', ref(''))
const path = inject<Ref<string>>('path', ref(''))

const defaultPathPrefix = hostName.value + path.value
const clientTokenId = ref<string>('')
clientTokenId.value = (Array.isArray(clientToken) ? clientToken[0] : clientToken) as string || '';
const intervalId = ref(0)
const jobIntervalId = ref(0)
const filePaths = ref<string[]>(["F:\\抢票\\spider_img", "D:\\抢票\\spider_img",])
const filePath = ref(filePaths.value[0])
const authFile = "auth.json"
const indexFile = "index.json"
const memberFile = "member.json"
const userInfoFiles = [authFile, indexFile, memberFile]
const sendOrderStatus = ref(false)
const jobs = ref([])


interface UserInfo {
  id: number;
  uid: number;
  realName: string;
  real_card_id: string;
  phone: string;
  is_self: boolean;
  real_card_id2: string;
  phone2: string;
  timestamp: number;
  signature: string;
}

interface UserInfoView {
  userId: string;
  loginCode: string;
  token: string;
  members: UserInfo[];
  users: string;
  userNames: string;
  iv: string;
  encryptKey: string;
  version: number;
  expireTime: number;
  expireTimeStr: string;
  regions: string;
  expire: boolean;
  memberIdNames: string
}

const userInfoViews = ref<UserInfoView[]>([])
const currentBindUserId = ref<string>('')

function createEmptyUserInfoView(): UserInfoView {
  return {
    userId: '',
    loginCode: '',
    token: '',
    members: [],
    users: '',
    userNames: '',
    iv: '',
    encryptKey: '',
    version: 0,
    expireTime: 0,
    expireTimeStr: "",
    regions: '',
    expire: false,
    memberIdNames: ""
  };
}

const findUserById = (userId: string, members: UserInfo[]): UserInfo | null => {
  const numericId = parseInt(userId, 10);
  return members.find(member => member.id === numericId) || null;
};


const currentBindUserInfoView = ref<UserInfoView>(createEmptyUserInfoView())
const getUserInfos = async () => {
  try {
    const response = await axios.get(`${defaultPathPrefix}/order/getUserInfos`);
    return response.data;
  } catch (error) {
    console.error('Error fetching user infos:', error);
    throw error;
  }
};

onMounted(() => {
  getUserInfos().then(userInfos => {
    console.log('Fetched user infos:', userInfos);
    userInfoViews.value = userInfos;
  })
})

const ALL_REGION = "全部区域"

function buildCurrentBindUserInfoView(selectedUser: UserInfoView) {
  currentBindUserInfoView.value = createEmptyUserInfoView()
  currentBindUserInfoView.value = selectedUser;
  if (selectedUser.regions == "") {
    currentBindUserInfoView.value.regions = ALL_REGION
  }
  currentBindUserInfoView.value.memberIdNames = selectedUser.members.map(user => `${user.id} ${user.realName}`).join(",")


  if (selectedUser.users != "") {
    const idsArray = selectedUser.users.split(',').map(id => id.trim());
    const namesArray = idsArray.map(id => {
      const user = findUserById(id, selectedUser.members);
      return user ? user.realName : id;
    });
    currentBindUserInfoView.value.userNames = namesArray.join(', ');
    currentBindUserInfoView.value.expireTimeStr = formatTimestamp(currentBindUserInfoView.value.expireTime)
    timeDifference.value = calculateTimeDifference(new Date(currentBindUserInfoView.value.expireTime), new Date())
  }
}

// 处理用户选择的方法
const onUserChange = (userId: string) => {

  const selectedUser = userInfoViews.value.find(user => user.userId === userId);
  if (selectedUser) {
    buildCurrentBindUserInfoView(selectedUser);
  } else {
    currentBindUserInfoView.value = createEmptyUserInfoView();
  }
};

// 格式化时间的函数
const formatCurrentTime = (): string => {
  const date = new Date();
  const yyyy = date.getFullYear();
  const MM = String(date.getMonth() + 1).padStart(2, '0'); // getMonth() 返回 0-11，需要加 1
  const dd = String(date.getDate()).padStart(2, '0');
  const HH = String(date.getHours()).padStart(2, '0');
  const mm = String(date.getMinutes()).padStart(2, '0');
  const ss = String(date.getSeconds()).padStart(2, '0');
  return `${yyyy}-${MM}-${dd} ${HH}:${mm}:${ss}`;
};

// 计算两个时间之间的差异（以小时、分钟、秒为单位）
const calculateTimeDifference = (start: Date, end: Date): string => {
  const diff = end.getTime() - start.getTime(); // 毫秒数差异
  const hours = Math.floor(diff / (1000 * 60 * 60));
  const minutes = Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60));
  const seconds = Math.floor((diff % (1000 * 60)) / 1000);
  return `${hours}小时 ${minutes}分钟 ${seconds}秒`;
};
// 定义一个 ref 来保存时间差
const timeDifference = ref<string>("");


// 定义一个 ref 来保存当前时间
const currentTime = ref<string>(formatCurrentTime());

// 使用 setInterval 每秒更新当前时间
setInterval(() => {
  currentTime.value = formatCurrentTime();
  if (currentBindUserInfoView.value.expireTime != 0) {
    timeDifference.value = calculateTimeDifference(new Date(currentBindUserInfoView.value.expireTime), new Date())
  }
}, 1000);

// 格式化时间戳
const formatTimestamp = (timestamp: number): string => {
  const date = new Date(timestamp);
  const yyyy = date.getFullYear();
  const MM = String(date.getMonth() + 1).padStart(2, '0'); // getMonth() 返回 0-11，需要加 1
  const dd = String(date.getDate()).padStart(2, '0');
  const HH = String(date.getHours()).padStart(2, '0');
  const mm = String(date.getMinutes()).padStart(2, '0');
  const ss = String(date.getSeconds()).padStart(2, '0');
  return `${yyyy}-${MM}-${dd} ${HH}:${mm}:${ss}`;
};


interface UserInfoRequest {
  userId: string,
  member: string,
  loginCode: string,
  token: string
}


interface OrderInfo {
  orderId: string,
  userId: string,
  realName: string
}

const candidateOrderInfos = ref<Map<string, OrderInfo[]>>(new Map([]))
const orderInfos = ref<Map<string, OrderInfo[]>>(new Map([]))

const checkedOrderIds = ref<string[]>([]);

const prepareDeleteOrderIds = ref<string[]>([]);
const savedOrderIds = ref<string[]>([]);

interface CheckUserBtn {
  btnStatus: boolean
}

const checkUserBtn = ref<CheckUserBtn>({btnStatus: false});

async function checkUserInfo() {

  let folderPath = filePath.value
  if (folderPath.length == 0) {
    open4()
    return
  }
  checkUserBtn.value.btnStatus = true

  sendOrderStatus.value = true

  intervalId.value = setInterval(sendUseInfo, 1000);
  jobIntervalId.value = setInterval(getJobs, 5000);
  getUserInfos().then(userInfos => {
    console.log('Fetched user infos:', userInfos);
    userInfoViews.value = userInfos;
  })

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
    axios.post(`${defaultPathPrefix}/order/createUserInfo`, userInfoRequest)
    for (const file of userInfoFiles) {
      const newFilePath = await join(folderPath + userId + '\\', file);
      const targetDir = await dirname(newFilePath);
      await createDir(targetDir, {recursive: true});
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
  axios.post(`${defaultPathPrefix}/order/createOrders`, orderIds)
      .then(() => {
        getJobs()
      })
}


function bindUserInfo() {
  const bindUserInfoReq = currentBindUserInfoView.value
  if (bindUserInfoReq.regions == ALL_REGION) {
    bindUserInfoReq.regions = ""
  }
  axios.post(`${defaultPathPrefix}/order/bindUserInfo`, bindUserInfoReq).then(resp => {
    buildCurrentBindUserInfoView(resp.data)
  })
}

function deleteOrders() {
  const orderIds = prepareDeleteOrderIds.value
  axios.post(`${defaultPathPrefix}/order/deleteOrders`, orderIds).then(() => {
    getJobs()
  })

}

function getJobs() {
  const runningJobs = axios.get(`${defaultPathPrefix}/order/getJobs`)
  runningJobs.then(response => {
    jobs.value = response.data
  })
}

function getUserCandidateOrders() {
  axios.get(`${defaultPathPrefix}/order/getUserCandidateOrders`).then(response => {
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
  axios.get(`${defaultPathPrefix}/order/getUserOrders`).then(response => {
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
  checkUserBtn.value.btnStatus = false
}


const open4 = () => {
  ElNotification({
    title: '请输入文件路径',
    message: '',
    type: 'error',
  })
}

const goBack = () => {
  router.push({name: 'Login'});
};
</script>

<template>
  <div class="content">
    <div class="formContainer">
      <el-form>
        <el-form-item label="服务信息: " label-width="100px">
          <el-input v-model="defaultPathPrefix" :placeholder="defaultPathPrefix" readonly class="readonly-input"/>
        </el-form-item>
        <el-form-item label="文件夹路径: " label-width="100px">
          <el-select v-model="filePath" placeholder="请选择文件夹路径" @change="onUserChange">
            <el-option
                v-for="filePath in filePaths"
                :key="filePath"
                :label="filePath"
                :value="filePath">
            </el-option>
          </el-select>
        </el-form-item>
      </el-form>
      <div class="button-container">
        <el-button v-show="sendOrderStatus" type="danger" @click="stopRead">停止监测</el-button>
        <el-button type="primary" :disabled="checkUserBtn.btnStatus" @click="checkUserInfo">开始监听新用户</el-button>
      </div>
      <hr/>
      <el-form>
        <el-form-item label="选择用户: " label-width="100px">
          <el-select v-model="currentBindUserId" placeholder="请选择用户" @change="onUserChange">
            <el-option
                v-for="user in userInfoViews"
                :key="user.userId"
                :label="user.members[0].realName"
                :value="user.userId">
            </el-option>
          </el-select>
        </el-form-item>
        <el-form-item label="用户id: " label-width="100px">
          <el-input v-model="currentBindUserInfoView.userId" placeholder="用户id" clearable/>
        </el-form-item>
        <el-form-item label="成员列表: " label-width="100px">
          <el-input v-model="currentBindUserInfoView.memberIdNames" placeholder="成员列表" readonly
                    class="readonly-input"/>
        </el-form-item>
        <el-form-item label="抢票球迷id: " label-width="100px">
          <el-input v-model="currentBindUserInfoView.users" placeholder="抢票人员" clearable/>
        </el-form-item>
        <el-form-item label="球迷名称: " label-width="100px">
          <el-input v-model="currentBindUserInfoView.userNames" placeholder="userid" readonly class="readonly-input"/>
        </el-form-item>
        <el-form-item label="加密密钥: " label-width="100px">
          <el-input v-model="currentBindUserInfoView.encryptKey" placeholder="加密密钥" clearable/>
        </el-form-item>
        <el-form-item label="过期时间: " label-width="100px">
          <el-input v-model="currentBindUserInfoView.expireTime" placeholder="过期时间" clearable/>
          <el-input v-model="currentBindUserInfoView.expireTimeStr" placeholder="过期时间" readonly
                    class="readonly-input"
          />
          <el-input v-model="currentTime" readonly class="readonly-input"/>
          <el-input v-model="timeDifference" readonly class="readonly-input"/>
        </el-form-item>
        <el-form-item label="IV: " label-width="100px">
          <el-input v-model="currentBindUserInfoView.iv" placeholder="IV" clearable/>
        </el-form-item>
        <el-form-item label="版本: " label-width="100px">
          <el-input v-model="currentBindUserInfoView.version" placeholder="版本" clearable/>
        </el-form-item>
        <el-form-item label="座位区域: " label-width="100px">
          <el-input v-model="currentBindUserInfoView.regions" placeholder="地区" clearable/>
        </el-form-item>
        <el-form-item label="鉴权token: " label-width="100px">
          <el-input v-model="currentBindUserInfoView.token" placeholder="鉴权token" clearable/>
        </el-form-item>
        <el-form-item label="loginCode: " label-width="100px">
          <el-input v-model="currentBindUserInfoView.loginCode" placeholder="loginCode" clearable/>
        </el-form-item>
      </el-form>

      <div class="button-container">
        <el-button type="primary" @click="bindUserInfo">绑定用户关键信息</el-button>
      </div>
      <hr/>
      <div class="formContainer checkbox-container">

        <div v-for="[userId, userInfos] in candidateOrderInfos" :key="userId" :value="userId">
          <h3>{{ userId }}</h3>
          <el-checkbox-group v-model="checkedOrderIds">
            <el-checkbox v-for="orderInfo in userInfos" :key="orderInfo.orderId" :value="orderInfo.orderId">
              {{ orderInfo.orderId }} {{ orderInfo.realName }}
            </el-checkbox>

          </el-checkbox-group>
        </div>


        <div class="button-container">
          <el-button type="primary" @click="getUserCandidateOrders">刷新候选订单</el-button>
          <el-button type="primary" @click="createOrders">创建订单</el-button>
        </div>
      </div>
      <hr/>
      <div class="formContainer checkbox-container">

        <div v-for="[userId, userInfos] in orderInfos" :key="userId" :value="userId">
          <h3>{{ userId }}</h3>
          <el-checkbox-group v-model="savedOrderIds">
            <el-checkbox v-for="orderInfo in userInfos" :key="orderInfo.orderId" :value="orderInfo.orderId">
              {{ orderInfo.orderId }} {{ orderInfo.realName }}
            </el-checkbox>

          </el-checkbox-group>
        </div>


        <div class="button-container">
          <el-button type="primary" @click="getUserOrders">查看已经保存的订单</el-button>
        </div>
      </div>
      <hr/>

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
      <hr/>
      <div class="btn">
        <el-button @click="goBack">返回</el-button>
      </div>
    </div>
  </div>
</template>

<style scoped lang="scss">
.content {
  width: 80%;
  .formContainer {
    display: flex;
    justify-content: center;
    flex-direction: column;
    align-items: center;
    width: 750px;

    .el-form {
      width: 80%; /* 设置表单宽度 */
      margin: 0 auto; /* 水平居中 */

      .el-form-item {
        width: 100%;

        .readonly-input ::v-deep .el-input__wrapper {
          background-color: #f5f5f5 !important; /* 灰色背景 */
          border-color: #dcdcdc !important; /* 灰色边框，可选 */
          color: #888 !important; /* 字体颜色，可选 */
        }
      }
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
      margin-bottom: 10px;
      flex-wrap: wrap;

      .el-checkbox {
        width: 150px;
      }
    }
  }
}


</style>