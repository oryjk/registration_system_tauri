<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
// import FileReader from './components/FileReader.vue'
import { ref } from "vue"

import { useRouter } from 'vue-router';
import Working from './components/Working.vue';
import CheckToken from './components/CheckToken.vue';
import axios from 'axios'


const router = useRouter();

const form = ref({
  token: ''
})

const isLogin = ref(false)
const hostName = "https://oryjk.cn:82"

function login() {
  const res = CheckToken.login()

  console.log(res)
  if (Boolean(res.valid) == true) {
    isLogin.value = true
  }

}

</script>

<template>
  <div class="container">
    <CheckToken v-if="!isLogin" />
    <Working v-if="isLogin" />
    <el-button type="primary" @click="login">登录</el-button>
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
