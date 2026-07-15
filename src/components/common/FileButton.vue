<template>
  <div>
    <button 
        class="upload-btn"
        @click="handleSelectFolder"
      >
        选择文件夹
      </button>
      
      <div class="info-text">
        <p class="filename">文件夹名称: {{ folderName || '未选择' }}</p>
        <p class="file-count">图片共计: {{ imageCount }} 张</p>
      </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const handleSelectFolder = ()=>{
    console.log("file choose")
}
const folderName = ref('');
const imageCount = ref(0);
</script>

<style scoped>
.upload-btn {
  padding: 16px 80px;
  font-size: 18px;
  margin-bottom: 12px;
}
.info-text {
  color: #717171;
  font-size: 13px;
}
</style>