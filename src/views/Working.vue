<template>
  <div class="container">
    <div class="btn">
      <el-button @click="goBack">返回</el-button>
    </div>
    <el-tabs type="border-card" class="fixed-tabs" v-model="activeName">
      <el-tab-pane label="订单情况" name="first">
        <OrderLists/>
      </el-tab-pane>

      <el-tab-pane label="订单直接录入" name="second">
        <OrderForm/>
      </el-tab-pane>

      <el-tab-pane label="用户信息录入" name="third">
        <UserInfoForm/>
      </el-tab-pane>

    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import {ref} from 'vue';
import UserInfoForm from '../components/UserInfoForm.vue';
import OrderForm from '../components/OrderForm.vue';
import OrderLists from "../components/OrderLists.vue";
import {useRoute, useRouter} from "vue-router";

useRoute();
const router = useRouter()
const activeName = ref('first'); // 将 activeName 的初始值设置为 "first"
const goBack = () => {
  router.push({name: 'Login'});
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
  //height: 100vh; /*设置容器高度 */
  overflow-y: auto; /* 允许垂直滚动 */

  .fixed-tabs {
    //position: fixed;
    top: 0;
    width: 80%;
    z-index: 100; /* 确保 Tab 栏在其他内容之上 */
    background-color: white; /* 避免背景透明 */
  }
}
</style>