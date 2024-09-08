<template>
    <div>
      <h1>About Page</h1>
      <button @click="goToHome">Go to Home</button>
    </div>
  </template>
  
  <script lang="ts" setup>
  import { useRouter } from 'vue-router';
  
  const router = useRouter();
  
  function goToHome() {
    router.push('/');
  }
  </script>
  