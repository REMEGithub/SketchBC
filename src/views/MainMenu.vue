<template>
  <div class="main-container">
    <!-- Logo 区域 -->
    <div class="logo-section">
      <img 
        src="@/assets/logo.png" 
        alt="Logo" 
        class="logo"
      >
    </div>

    <!-- 选择文件夹区域 -->
    <div class="folder-section">
      <FileButton/>
    </div>

    <!-- 分隔线 + 绘画模式选择 -->
    <div class="mode-section">
      <div class="divider"></div>
      <h3 class="mode-title">绘画模式选择</h3>
    </div>

    <ToggleButtonGroup/>

    <!-- 参数区域（可扩展） -->
    <div class="param-area">
      <!-- 自定义模式时在这里显示额外参数 -->
      <slot name="params"></slot>
    </div>

    <!-- 开始绘制按钮 -->
    <PageJumpButton label="开始" :to="pname" @click="startDrawing" />
  </div>
</template>

<script setup lang="ts">
import FileButton from '@/components/common/FileButton.vue'
import ToggleButtonGroup from '@/components/common/ToggleButtonGroup.vue';
import PageJumpButton from '@/components/common/PageJumpButton.vue';
const pname = {name:'exhibit'}


const startDrawing = () => {
  // console.log('开始绘制', selectedMode.value)
  // 后面调用 Rust 或打开新窗口
}
</script>

<style scoped>
.main-container {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 40px 20px;
  overflow: hidden;
  background: #f8f9fa;
  box-sizing: border-box;
}

.logo-section {
  margin-bottom: 40px;
}

.logo {
  max-width: 250px;
  height: auto;
}

.folder-section {
  margin-bottom: 30px;
  text-align: center;
}

.mode-section {
  margin: 20px 0;
  text-align: center;
}

.divider {
  width: 500px;
  height: 1px;
  background: #ddd;
  margin: 0 auto 12px;
}

.mode-title {
  font-size: 18px;
  margin: 0;
  color: #333;
}



.start-section {
  margin-top: auto;
}

.start-btn {
  padding: 18px 120px;
  font-size: 22px;
  font-weight: 500;
  background: #007aff;
  color: white;
  border: none;
  border-radius: 10px;
  cursor: pointer;
}
</style>