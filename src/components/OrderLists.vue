<script setup lang="ts">
import {inject, Ref, ref} from 'vue'
import axios from 'axios'

const hostName = inject<Ref<string>>('hostName', ref(''))
const path = inject<Ref<string>>('path', ref(''))

const defaultPathPrefix = hostName.value + path.value
const candidateOrderInfos = ref<UserOrderInfoView[]>([])
const fullscreenLoading = ref(false)

interface UserOrderInfoView {
  userId: string,
  realName: string,
  regions: string[]
}


function getUserCandidateOrders(number: number) {
  fullscreenLoading.value = true
  axios.get(`${defaultPathPrefix}/order/getUserCandidateOrdersByOrderRequest/${number}`).then(response => {
    console.log(response.data)
    candidateOrderInfos.value=[]
    candidateOrderInfos.value = response.data.map((user: UserOrderInfoView) => ({
      ...user,
      regions: [...user.regions].sort(), // Create a copy to avoid mutating the original data
    }));
    fullscreenLoading.value = false
  })
}
</script>

<template>
  <div class="formContainer checkbox-container">

    <div class="candidateOrderInfos">

      <el-card v-for="userOrderInfoView in candidateOrderInfos" :key="userOrderInfoView.userId"
               :value="userOrderInfoView" shadow="hover" style="width: 150px;height: 500px">
        <template #header>
          <div class="card-header">
            <p>id：{{ userOrderInfoView.userId }}</p>
            <p>名字：{{ userOrderInfoView.realName }}</p>
          </div>
        </template>
        <p v-for="region in userOrderInfoView.regions" :key="region" class="regionItem">
          座位： {{ region }}
        </p>
      </el-card>
    </div>


    <div class="button-container">
      <el-button v-loading.fullscreen.lock="fullscreenLoading"
                 type="primary"
                 @click="getUserCandidateOrders(0)">刷新进行的订单</el-button>
      <el-button v-loading.fullscreen.lock="fullscreenLoading"
                 type="primary"
                 @click="getUserCandidateOrders(1)">刷新已经抢到的订单</el-button>
      <el-button v-loading.fullscreen.lock="fullscreenLoading"
                 type="primary"
                 @click="getUserCandidateOrders(2)">刷新已经过期的订单</el-button>

    </div>
  </div>
</template>

<style scoped lang="scss">
.formContainer {
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
  width: 750px;

  .candidateOrderInfos {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    width: 100%;
    flex-wrap: wrap;
    gap: 20px;

    .card-header{
      color: crimson;
      font-weight: bold;
    }
    .regionItem{
      color: #249b73;
    }
  }
}
</style>