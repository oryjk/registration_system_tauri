<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
// import Greet from "./components/Greet.vue";
// import FileReader from './components/FileReader.vue'
import { ref } from "vue"
import axios from 'axios'
import { useRouter } from 'vue-router';

const hostName = "https://oryjk.cn:82"
const router = useRouter();

const form = ref({
  token: ''
})

function login() {
  console.log(form.value.token)
  // router.push('/about');
  window.open(router.resolve('/about').href, '_blank');

  getData(hostName + "/rs/login?token=" + form.value.token)
    .then((res) => {
      console.log(res)
      if (Boolean(res) == true) {
        router.push('/');
      }
    })
}

async function getData(uri: string): Promise<string> {
  return (await axios.get(uri)).data
}
</script>

<template>
  <div class="container">
    <h1>信息采集</h1>
    <div class="form_container">

      <el-form-item label="客户端token">
        <el-input v-model="form.token" />
      </el-form-item>

      <el-button type="primary" @click="login">登录</el-button>

    </div>
    <nav>
      <router-link to="/">Home</router-link>
      <router-link to="/about">About</router-link>
    </nav>
    <router-view></router-view>
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
