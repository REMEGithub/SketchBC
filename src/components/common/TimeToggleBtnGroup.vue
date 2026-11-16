<template>
    <div class="timer-panel">

        <!-- 第一行：30s选项 + 时间统计框 -->
        <div class="timer-header">

            <!-- 30s按钮，依旧是正常的时间选项 -->
            <button type="button" class="timer-btn" :disabled="isTimeLocked" :class="{
                active: timer.selectedSeconds === 30
            }" @click="timer.setTime(30); drawingParam.setdisplaySeconds(30)">

                <svg class="svg-text" viewBox="0 0 144 68">
                    <text x="50%" y="50%" dominant-baseline="central" text-anchor="middle">
                        30s
                    </text>
                </svg>

            </button>

            <!-- 时间统计框 -->
            <div class="timer-display">
                {{ timer.formattedTime }}
            </div>

        </div>

        <!-- 下方时间选择区域 -->
        <div class="timer-options">
            <button v-for="item in timer.presets.slice(1)" :key="item.seconds" type="button" class="timer-btn" :disabled="isTimeLocked" :class="{
                active: timer.selectedSeconds === item.seconds
            }" @click="timer.setTime(item.seconds); drawingParam.setdisplaySeconds(item.seconds)">

                <svg class="svg-text" viewBox="0 0 144 68">
                    <text x="50%" y="50%" dominant-baseline="central" text-anchor="middle">
                        {{ item.label }}
                    </text>
                </svg>

            </button>
        </div>

    </div>
</template>

<script setup lang="ts">
import { useTimerStore } from '@/stores/useTimeStore'
import { useDrawingParamStore } from '@/stores/useDrawingParamStore'
import { computed } from 'vue'

const timer = useTimerStore()
const drawingParam = useDrawingParamStore()

const isTimeLocked = computed(() => drawingParam.isTimeLocked)

</script>

<style scoped>
.timer-panel {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 8px 11px;
}

/* 第一行 */
.timer-header {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 11px;
    height: 68px;
}

/* 时间统计框 */
.timer-display {
    grid-column: span 2;

    display: flex;
    align-items: center;
    justify-content: center;

    background: #fff;
    border-radius: 10px;

    font-size: 26px;
    font-weight: 600;
    color: #111;

    white-space: nowrap;
}

/* 下方按钮组 */
.timer-options {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 8px 11px;
}

/* 所有时间按钮 */
.timer-btn {
    position: relative;
    height: 68px;
    border: none;
    border-radius: 8px;

    background: #c6c6c6;
    color: white;

    font-size: 27px;
    font-weight: bold;

    cursor: pointer;

    transition:
        background 0.2s ease,
        transform 0.15s ease;
}

/* SVG 文字 */
.svg-text {
    position: absolute;
    inset: 0;

    width: 100%;
    height: 100%;

    overflow: visible;
    pointer-events: none;
}

/* SVG 描边字体 */
.svg-text text {
    font-size: 30px;
    font-weight: 400;

    fill: rgba(195, 195, 195, 1);
    stroke: #fff;
    stroke-width: 15px;
    stroke-linejoin: round;
    stroke-linecap: round;

    paint-order: stroke fill;

    /* transition: fill 0.2s ease; */
}


/* 当前选中状态 */
.timer-btn.active {
    background: #d7f77c;

}
.timer-btn.active .svg-text text{
    fill: rgb(0, 0, 0);
}

/* 悬停 */
.timer-btn:not(.active):hover {
    background: #b5b5b5;
}

/* 点击 */
.timer-btn:active {
    transform: scale(0.97);
}
</style>
