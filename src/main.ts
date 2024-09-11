import { createApp } from "vue";
import App from "./App.vue";
import ElementPlus from 'element-plus'
// import router from './router';
import 'element-plus/dist/index.css'

const app = createApp(App)

// app.use(router);
app.use(ElementPlus)
app.config.globalProperties.$hostName = "https://oryjk.cn:82"
app.config.globalProperties.$clientToken = "1wscEia6CidvrxbmpOdWp5SviRKjIQy8"
app.mount('#app')

