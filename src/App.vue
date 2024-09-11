<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
// import FileReader from './components/FileReader.vue'
import { ref, provide } from "vue"

import Working from './components/Working.vue';
import CheckToken from './components/CheckToken.vue';
import { ElNotification } from 'element-plus'

const hostName = 'https://oryjk.cn:82';
const clientToken = '1wscEia6CidvrxbmpOdWp5SviRKjIQy8';
provide('clientToken', clientToken);
provide('hostName', hostName);
const isLogin = ref(false)

const checkToken = ref();
const working = ref();



function login() {
  const res = checkToken.value?.login()

  res.then((tokenInfo: any) => {
    if (Boolean(tokenInfo.valid) == true) {
      isLogin.value = true
    } else {
      ElNotification({
            title: 'token输入不正确',
            message: '请检查后重新输入',
            type: 'error',
        })
    }
  })


}

</script>

<template>
  <div class="container">
    <div v-if="!isLogin">
      <CheckToken ref="checkToken" />
      <el-button type="primary" @click="login">登录</el-button>
    </div>


    <Working ref="working" v-if="isLogin" />



  </div>

</template>

<style scoped lang="scss">
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;

  .form_container {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }
}
</style>
