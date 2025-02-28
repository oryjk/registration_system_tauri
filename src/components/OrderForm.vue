<script setup lang="ts">
import {computed, inject, onMounted, ref, Ref} from 'vue';

import axios from "axios";
import {ElNotification} from 'element-plus'
import { ElForm } from 'element-plus'; // 引入 ElForm 类型

const hostName = inject<Ref<string>>('hostName', ref(''))
const path = inject<Ref<string>>('path', ref(''))
type RegionMap = Record<string, Region>;
const regionData = ref<RegionMap>({}); // 存储 Region 数据的响应式对象
const matchData = ref<Match>({awayName: "", date: new Date(), homeName: "", hour: 0, matchId: "", minute: 0, round: 0});
const selectedRegionNames = ref<string>(''); // 存储选中的 Region 的名称
// 定义表单数据模型
const form = ref({
  orderId: '',
  id: '',
  loginCode: '',
  matchId: '',
  orderPayload: '',
  order_status: 0,
  token: '',
  clientTokenId: 'MUX9cBF8',
  uid: '',
  region: ''
});


interface Region {
  region: number;
  estate: number;
  num: number;
  name: string;
  price: string;
  usable_count: number;
  key:string
}

interface Match {
  date: Date,
  hour: number,
  minute: number,
  homeName: string,
  awayName: string,
  round: number,
  matchId: string
}

const regionChange = (value: string) => {
  form.value.region = value
}

const ruleFormRef = ref<typeof ElForm | null>(null); // 明确类型

const rules = ref({
  region: [{required: true, message: '请选择区域', trigger: 'change'}],
  id: [{required: true, message: '请输入用户ID', trigger: 'blur'}],
  uid: [{required: true, message: '请输入UID', trigger: 'blur'}],
  loginCode: [{required: true, message: '请输入Login Code', trigger: 'blur'}],
  matchId: [{required: true, message: '请输入Match ID', trigger: 'blur'}],
  orderPayload: [{required: true, message: '请输入Order Payload', trigger: 'blur'}],
  token: [
    {required: true, message: '请输入Token', trigger: 'blur'},
    {
      validator: (_:any, value:string, callback: (error?: Error) => void) => {
        if (typeof value === 'string' && !value.startsWith('Bearer ')) {
          callback(new Error('Token 必须以 "Bearer " 开头'));
        } else {
          callback();
        }
      },
      trigger: 'blur',
    },
  ],
  clientTokenId: [{required: true, message: '请输入Token ID', trigger: 'blur'}]

});


// 发送网络请求获取 Region 数据
const fetchRegionData = async () => {
  try {
    const response = await axios.get<RegionMap>(hostName.value + path.value + '/common/all-ticket'); // 替换为你的 API 端点
    regionData.value = response.data;
  } catch (error) {
    console.error('Error fetching region data:', error);
    // 处理错误，例如显示错误提示
  }
};

// 发送网络请求获取 Region 数据
const fetchMatchInfo = async () => {
  try {
    const response = await axios.get<Match>(hostName.value + path.value + '/schedule/current'); // 替换为你的 API 端点
    matchData.value = response.data;
    form.value.matchId = matchData.value.matchId;
  } catch (error) {
    console.error('Error fetching region data:', error);
    // 处理错误，例如显示错误提示
  }
};
// 组件挂载后，获取 Region 数据
onMounted(() => {
  fetchRegionData();
  fetchMatchInfo();
});

interface ClientOrderRequest {
  orderId: string;
  matchId: string;
  orderPayload: string;
  loginCode: string;
  token: string;
  clientTokenId: string;
  uid: string;
}


const open1 = () => {
  ElNotification({
    title: 'Success',
    message: 'This is a success message',
    type: 'success',
  })
}
const open2 = () => {
  ElNotification({
    title: 'Warning',
    message: 'This is a warning message',
    type: 'warning',
  })
}

const onSubmit = () => {
  if(ruleFormRef.value){
    ruleFormRef.value.validate(async (valid:any) => {
      if (valid) {
        try {
          const currentForm = ref({...form.value, orderId: selectedRegionNames.value});
          const orderRequest: ClientOrderRequest = {
            orderId: currentForm.value.id + `|${currentForm.value.matchId}|` + currentForm.value.orderId,
            matchId: currentForm.value.matchId,
            orderPayload: currentForm.value.orderPayload,
            loginCode: currentForm.value.loginCode,
            token: currentForm.value.token,
            clientTokenId: currentForm.value.clientTokenId,
            uid: currentForm.value.uid,
          };
          const response = await axios.post(hostName.value + path.value + '/order/createSimpleOrder', orderRequest);
          console.log('提交成功:', response.data);
          open1()
        } catch (error) {
          console.error('提交失败:', error);
          open2()
        }
      } else {
        console.log('error submit!');
        return false;
      }
    });
  }

};

// 重置表单
const onReset = () => {
  form.value = {
    orderId: '',
    id: '',
    loginCode: '',
    matchId: '',
    orderPayload: '',
    order_status: 0,
    token: '',
    clientTokenId: 'MUX9cBF8',
    uid: '',
    region: ''
  };
};

// 计算属性：按照 price 分组的 Region 数据
const groupedRegions = computed(() => {
  const grouped: Record<string, Region[]> = {};
  for (const key in regionData.value) {
    if (regionData.value.hasOwnProperty(key)) {
      const region = regionData.value[key];
      if (!grouped[region.price]) {
        grouped[region.price] = [];
      }
      grouped[region.price].push({...region, key}); // 添加 key 属性，方便排序
    }
  }

  // 将 grouped 对象转换为数组，并按照 price 排序
  return Object.entries(grouped)
      .sort(([, regionsA], [, regionsB]) => {
        const priceA = parseFloat(regionsA[0].price); // 将价格转换为数字
        const priceB = parseFloat(regionsB[0].price);
        return priceA - priceB; // 按照价格升序排序
      })
      .reduce((obj: Record<string, Region[]>, [key, value]) => {
        obj[key] = value;
        return obj;
      }, {});
});


// 方法：用于在每个分组内按照名称排序
const sortedRegions = (regions: Region[]) => {
  return [...regions].sort((a, b) => a.name.localeCompare(b.name));
};

</script>

<template>
  <div class="content">
    <div class="formContainer">
      <div class="region-list">
        <!-- 按照 price 分组显示 Region 数据 -->
        <div v-for="(regions, price) in groupedRegions" :key="price" class="region-group">
          <h3>价格：{{ price }}</h3>
          <!-- 按照 name 排序并使用 checkbox 多选 -->
          <el-radio-group v-model="selectedRegionNames" @change="regionChange">
            <el-radio
                v-for="region in sortedRegions(regions)"
                :key="region"
                :label="region.name"
            >{{ region.name }} (参考余票{{ region.usable_count }})
            </el-radio>
          </el-radio-group>
        </div>
      </div>
      <el-form :model="form" :rules="rules" label-width="120px" ref="ruleFormRef">
        <el-form-item label="座位号" prop="region">
          <el-input v-model="form.region" placeholder="请选择座位号" disabled/>
        </el-form-item>
        <el-form-item label="比赛ID" prop="matchId">
          <el-input v-model="form.matchId" placeholder="请输入Match ID"/>
        </el-form-item>
        <el-form-item label="Login Code" prop="loginCode">
          <el-input v-model="form.loginCode" placeholder="请输入Login Code"/>
        </el-form-item>
        <el-form-item label="Token" prop="token">
          <el-input v-model="form.token" type="textarea" placeholder="请输入Token"/>
        </el-form-item>
        <el-form-item label="订单Payload" prop="orderPayload">
          <el-input v-model="form.orderPayload" type="textarea" placeholder="请输入Order Payload"/>
        </el-form-item>
        <el-form-item label="用户ID" prop="id">
          <el-input v-model="form.id" placeholder="请输入ID"/>
        </el-form-item>

        <el-form-item label="UID" prop="uid">
          <el-input v-model="form.uid" placeholder="请输入UID"/>
        </el-form-item>
        <el-form-item label="分销ID" prop="clientTokenId">
          <el-input v-model="form.clientTokenId" placeholder="请输入Token ID"/>
        </el-form-item>
        <el-form-item>
          <el-button type="primary" @click="onSubmit">提交</el-button>
          <el-button @click="onReset">重置</el-button>
        </el-form-item>
      </el-form>
    </div>
  </div>


</template>

<style scoped lang="scss">
.content {
  width: 100%;

  .formContainer {
    display: flex;
    justify-content: center;
    flex-direction: column;
    align-items: center;
    width: 100%;

    .region-list {
      margin-bottom: 20px;
    }

    .region-group {
      margin-bottom: 15px;
    }

    .el-checkbox-group {
      display: flex;
      flex-wrap: wrap;
      align-items: center;
      justify-content: center;
    }

    .el-checkbox {
      margin-right: 10px; /* 添加一些间距 */
      margin-bottom: 5px;
    }


    .el-form {
      width: 80%; /* 设置表单宽度 */
      margin: 0 auto; /* 水平居中 */
    }
  }
}


</style>