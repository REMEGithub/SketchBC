<template>
    <!-- 模式切换按钮 -->
    <div class="toggle-group">
      <button 
        v-for="mode in modes" 
        :key="mode.value"
        class="toggle-btn"
        :class="{ active: currentMode === mode.value }"
        @click="selectMode(mode.value)"
      >
        {{ mode.label }}
      </button>
    </div>
</template>

<script setup lang="ts">
import { useModeStore } from '@/stores/useModeStore';
import { storeToRefs } from 'pinia';

const modeStore = useModeStore();
const { currentMode } = storeToRefs(modeStore);


type ModeType = 'quick' | 'custom' | 'random'
const modes: { label: string; value: ModeType }[] = [
  { label: '快速模式', value: 'quick' },
  { label: '自定模式', value: 'custom' },
  { label: '随机模式', value: 'random' }
]


const selectMode = (mode: ModeType) => {
  currentMode.value = mode
}
</script>

<style scoped>

.toggle-group {
  display: flex;
  gap: 30px;
  margin-bottom: 40px;
}

.toggle-btn {
  padding: 16px 40px;
  font-size: 16px;
  border: 2px solid #ddd;
  background: white;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.toggle-btn.active {
  border-color: #007aff;
  background: #1a5ca3;
  color: white;
}
</style>