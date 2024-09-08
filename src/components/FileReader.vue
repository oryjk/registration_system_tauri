<template>
    <div>
        <button @click="readFile">读取文件</button>
        <div v-if="fileContent">
            <h3>文件内容：</h3>
            <pre>{{ fileContent }}</pre>
        </div>
    </div>
</template>

<script setup lang="ts">
import { readTextFile } from '@tauri-apps/api/fs';
import { ref } from 'vue'

const fileContent = ref < string | null > (null);

async function readFile() {
    const content = await readTextFile("E:\\抢票\\spider_img\\auth.json");
    console.log("hhhhhhhhh");
    
    fileContent.value = content
}


</script>

<style scoped>
/* 添加一些样式 */
</style>