<template>
  <div>
    <button class="upload-btn" @click="handleSelectFolder">
      <!-- Logo -->
      <img class="btn-logo" src="@/assets/imgSrc/home_cat1.png" alt="logo" />

      <!-- 文字信息 -->
      <div class="btn-info">
          <!-- 对应尺寸 231 * 51 -->
          <svg class="svg-text svg-title" viewBox="0 0 231 51">
            <text x="107" y="50%" dominant-baseline="central" text-anchor="start">{{folderName||'请选择文件夹'}}</text>
          </svg>
          
          <!-- 对应尺寸 90 * 29 -->
          <svg class="svg-text svg-desc" viewBox="0 0 90 29">
            <text x="107" y="50%" dominant-baseline="central" text-anchor="start">共 {{ imageCount }} 张图片</text>
          </svg>
      </div>

    </button>
      
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

import { useDrawingParamStore } from '@/stores/useDrawingParamStore';

const store = useDrawingParamStore();


const folderName = ref('');
const imageCount = ref(0);

const loading = ref(false);
const errorMessage = ref('');




// 选择文件夹
const handleSelectFolder = async () => {
  if (loading.value) return;

  try {
    errorMessage.value = '';

    // 1. 打开系统文件夹选择器
    const selected = await open({
      directory: true,
      multiple: false,
      title: '选择图片文件夹'
    });

    // 用户取消
    if (!selected) return;

    loading.value = true;

    // 2. Rust 扫描文件夹
    const result = await invoke<{
      folder_name: string;
      image_count: number;
    }>('set_image_folder', {
      folderPath: selected
    });

    // 3. 更新组件显示
    folderName.value = result.folder_name;
    imageCount.value = result.image_count;

    // 4. 同步到 Pinia
    store.setFolderInfo(
      result.folder_name,
      result.image_count
    );

    // 5. 检查是否没有图片
    if (result.image_count === 0) {
      errorMessage.value = '当前文件夹没有支持的图片文件';
    }

  } catch (error) {
    console.error('文件夹读取失败:', error);
    errorMessage.value = '文件夹读取失败，请重新选择';
  } finally {
    loading.value = false;
  }
  
};
</script>

<style scoped>
.upload-btn {
  display: flex;
  width: 709px;
  height: 174px;
  background-image: url('@/assets/imgSrc/home_button_L.png');
  background-size: 100% 100%;
  background-repeat: no-repeat;
  background-position: center;

  border: none;
  border-radius: 16px;
  cursor: pointer;
}
.btn-logo {
  width: 156px;
  height: 156px;
  margin:9px 0 0 60px;
}
.btn-info {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: flex-start;
  gap: 24px;
}


.svg-text {
  display: block;
  overflow: visible;
}

/* 严格锁定外部尺寸 */
.svg-title {
  width: 231px;
  height: 51px;
}

.svg-desc {
  width: 90px;
  height: 29px;
}

/* 公共描边样式 */
.svg-text text {
  font-weight: 400;
  fill: #000;               
  stroke: #fff;             
  stroke-width: 15px;        
  stroke-linejoin: round;   
  stroke-linecap: round;    
  paint-order: stroke fill; 
}

/* 单独控制字号 */
.svg-title text {
  font-size: 42px; 
  font-weight: 400;
  letter-spacing: 4px; /* Home 比较宽，字距拉开 */
}

.svg-desc text {
  font-size: 24px; 
  font-weight: 400;
  letter-spacing: 1px;
}


.info-text {
  color: #717171;
  font-size: 13px;
}
</style>