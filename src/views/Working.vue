<template>
  <div class="container">
    <el-tabs type="border-card" class="fixed-tabs" v-model="activeName">
      <el-tab-pane label="用户信息录入" name="first">
        <UserInfoForm/>
      </el-tab-pane>
      <el-tab-pane label="订单直接录入" name="second">
        <OrderForm/>
      </el-tab-pane>
    </el-tabs>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import UserInfoForm from '../components/UserInfoForm.vue';
import OrderForm from '../components/OrderForm.vue';
const activeName = ref('first'); // 将 activeName 的初始值设置为 "first"
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