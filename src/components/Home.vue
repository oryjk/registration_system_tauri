<template>
    <div>
      <h1>Home Page</h1>
      <button @click="goToAbout">Go to About</button>
    </div>
  </template>
  
  <script lang="ts" setup>
  import { useRouter } from 'vue-router';
  
  const router = useRouter();
  
  function goToAbout() {
    router.push('/about');
  }
  </script>
  