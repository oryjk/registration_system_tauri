<script setup lang="ts">
import {inject, Ref, ref} from "vue";
import axios from 'axios'

import {ElNotification} from 'element-plus'
import {useRouter} from 'vue-router';


const default_home_name = inject<Ref<string>>('hostName') || ref("https://oryjk.cn:82");
const default_api_path = inject<Ref<string>>('path') || ref("/ticket");

const host_names = ref<string[]>([default_home_name.value, "http://127.0.0.1:5689","http://172.16.60.237:5689"]);
const paths = ref<string[]>([default_api_path.value, "/api"]);


interface Config {
  token: string,
  hostName: string,
  apiPath: string
}

const form = ref<Config>({
  token: 'MUX9cBF8',
  hostName: default_home_name.value,
  apiPath: default_api_path.value,
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
  default_home_name.value = form.value.hostName
  default_api_path.value = form.value.apiPath
  let result
  try {
    result = await getData(form.value.hostName + form.value.apiPath + `/token/check/` + form.value.token)
  } catch (e) {
    if (axios.isAxiosError(e)) {
      ElNotification({
        title: '网络错误',
        message: '请检查服务器是否启动，并检查服务器日志是否有报错',
        type: 'error',
      })
      return
    }
  }
  if (!result || result.valid) {
    router.push({name: 'Working', query: {inviteCode: form.value.token}});
  } else {
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
      <el-form-item label="客户端token: " label-width="100px">
        <el-input :class="{ 'is-error': isError }" v-model="form.token" placeholder="客户端token:"/>
      </el-form-item>
      <el-form-item label="服务器地址: " label-width="100px">
        <el-select v-model="form.hostName" placeholder="服务器地址">
          <el-option
              v-for="host_name in host_names"
              :key="host_name"
              :label="host_name"
              :value="host_name">
          </el-option>
        </el-select>
      </el-form-item>
      <el-form-item label="path地址: " label-width="100px">
        <el-select v-model="form.apiPath" placeholder="服务器地址">
          <el-option
              v-for="path in paths"
              :key="path"
              :label="path"
              :value="path">
          </el-option>
        </el-select>
      </el-form-item>

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