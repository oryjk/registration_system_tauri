<script setup lang="ts">
import {inject, ref} from "vue";
import axios from 'axios'

import {ElNotification} from 'element-plus'
import {useRouter} from 'vue-router';


const hostName = inject<string>('hostName');


const form = ref({
  token: 'MUX9cBF8'
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
  let result = undefined
  try {
    result = await getData(hostName + "/ticket/token/check/" + form.value.token)
  } catch (e) {
    if (axios.isAxiosError(e) && e.code == "ERR_NETWORK") {
      ElNotification({
        title: '网络错误404',
        message: '无法连接到服务器，请稍后再试，或者查看服务器是否启动',
        type: 'error',
      })
      return
    }

  }

  if (!result || !result.valid) {
    ElNotification({
      title: 'token验证出错',
      message: '不是所期望的类型，没有valid属性',
      type: 'error',
    })
  } else {
    router.push({name: 'Working', query: {inviteCode: form.value.token}});
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
      <el-input :class="{ 'is-error': isError }" v-model="form.token" placeholder="客户端token:"/>
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