import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Login from '../views/Login.vue';
import Working from '../views/Working.vue';

const routes: Array<RouteRecordRaw> = [
  { path: '/', name: 'Login', component: Login },
  { path: '/user-details', name: 'Working', component: Working },
];

const router = createRouter({
  history: createWebHistory('/'),
  routes,
});

export default router;