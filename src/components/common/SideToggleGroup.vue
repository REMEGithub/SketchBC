<template>
    <div class="toggle-btn-group">
        <button
            v-for="mode in modes" 
            :key="mode.value"
            class="toggle-btn"
            :class="{ active: currentMode === mode.value }"
            @click="selectMode(mode.value)"
        >
            <!-- SVG：viewBox 锁定按钮尺寸 200x84 -->
            <svg class="svg-text" viewBox="0 0 200 84">
                <text x="50%" y="50%" dominant-baseline="central" text-anchor="middle">
                    {{ mode.label }}
                </text>
            </svg>
        </button>
    </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue';
import { useModeStore } from '@/stores/useModeStore';
import { useDrawingParamStore } from '@/stores/useDrawingParamStore';
import { useTimerStore } from '@/stores/useTimeStore';
import { storeToRefs } from 'pinia';

const modeStore = useModeStore();
const Dstore = useDrawingParamStore();
const Tstore = useTimerStore();
const { currentMode } = storeToRefs(modeStore);


type ModeType = 'quick' | 'custom' | 'random'
const modes: { label: string; value: ModeType }[] = [
  { label: '快速模式', value: 'quick' },
  { label: '自定模式', value: 'custom' },
  { label: '随机模式', value: 'random' }
]


const selectMode = (mode: ModeType) => {
  currentMode.value = mode
  if (mode === 'quick') {

    Dstore.setdisplaySeconds(10);  // 设置快速模式的默认时间为 30 秒
    Tstore.setTime(10);  // 设置快速模式的默认时间为 30 秒
    Dstore.setIsTimeLocked(true);  // 锁定时间选择

  } else if (mode === 'custom') {

    Dstore.setdisplaySeconds(0);
    Tstore.setTime(0);
    Dstore.setIsTimeLocked(false);  // 解锁时间选择


  } else if (mode === 'random') {

    const randomSeconds = Math.floor(Math.random() * 2701); // 生成 0 到 2700 秒之间的随机整数
    Dstore.setdisplaySeconds(randomSeconds);  // 设置随机模式的时间为 0 到 2700 秒之间的随机值
    Tstore.setTime(randomSeconds);
    Dstore.setIsTimeLocked(true);  // 锁定时间选择

  }
}

onMounted(() => {
  // 初始化模式为快速模式
  selectMode('quick');
});

</script>

<style scoped>
.toggle-btn-group {
    background: rgba(195, 195, 195, 1);
    height: 315px;
    border-radius: 16px;

    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
}

.toggle-btn {
    padding: 0;

    border: none;
    outline: none;
    appearance: none;

    width: 200px;
    height: 84px;
    margin: 6px;
    border-radius: 12px;
    background: transparent;
    cursor: pointer;

    /* 1. 设置相对定位，作为 SVG 的定位锚点 */
    position: relative; 
    display: flex;
    justify-content: center;
    align-items: center;
}

.toggle-btn.active {
    background: rgba(215, 246, 116, 1);
}

/* 2. SVG 绝对定位铺满整个按钮 */
.svg-text {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    overflow: visible; /* 允许 8px 描边溢出按钮边界 */
}

/* 3. 文字描边核心样式 */
.svg-text text {
    font-size: 32px;
    font-weight: 400;
    
    fill: rgba(195, 195, 195, 1);              
    stroke: #fff;             
    stroke-width: 15px;        
    stroke-linejoin: round;   
    stroke-linecap: round;    
    paint-order: stroke fill; 
}
.toggle-btn.active .svg-text text {
    fill: rgba(0, 0, 0, 1); /* 激活状态文字填充黑色 */
}
</style>