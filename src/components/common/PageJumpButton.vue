<template>
  <div class="start-section">
    <button class = base-btn
    :class = "[variant,size]"
    :disabled = "starting"
    @click="handleClick"
    >
      <svg class="svg-text" viewBox="0 0 454 84">
        <text 
          x="50%" 
          y="50%" 
          dominant-baseline="central" 
          text-anchor="middle"
        >{{ props.label }}</text>
      </svg>
    </button>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core'
import { getCurrentWindow } from '@tauri-apps/api/window'

import { useDrawingParamStore } from '@/stores/useDrawingParamStore';


const props = defineProps<{
    label: string;                // 按钮文字
    to?: string | object;         // 跳转路由（字符串或路由对象）
    variant?: 'primary' | 'secondary' | 'outline' | 'danger';  // 按钮样式
    size?: 'small' | 'medium' | 'large';                       // 按钮大小
    fullWidth?: boolean;          // 是否占满宽度
}>();

// const emit = defineEmits<{
//     (e:'click',event:MouseEvent):void
// }>();

const router = useRouter();

const starting = ref(false);

const store = useDrawingParamStore();


// const handleClick = (event:MouseEvent)=>{
//     if(props.disabled) return;

//     if(props.to){
//         router.push(props.to);
//     }else{
//         emit('click',event)
//     }
// }

const handleClick = async () => {
    if (starting.value) return

    try {
        starting.value = true

        // 检查展示时间
        if (
            !Number.isInteger(store.displaySeconds) ||
            store.displaySeconds < 1
        ) {
            alert('请输入有效的展示时间')
            return
        }

        // 从 Rust 获取真实图片数量
        const imageCount = await invoke<number>('get_image_count')

        if (imageCount === 0) {
            alert('请先选择包含图片的文件夹')
            return
        }

        // 同步图片数量
        store.imageCount = imageCount

        // 解除窗口大小锁定
        await getCurrentWindow().setResizable(true)

        // 跳转到图片轮播页面
        await router.push(props.to || '/exhibit')

    } catch (error) {
        console.error('开始绘画失败:', error)
        alert('无法启动图片轮播')
    } finally {
        starting.value = false
    }
}
</script>

<style scoped>
.start-section {
  margin-top: 11px;
}

.base-btn {
  width: 454px;
  height: 84px;
  border-radius: 12px;
  border: 6px solid rgba(195, 195, 195, 1);
  box-sizing: border-box;
  background: rgba(215, 246, 116, 1);
  cursor: pointer;

  /* 1. 父元素必须设置为相对定位 */
  position: relative; 
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 0; /* 清除默认 padding */
}
/* 默认 padding（medium 作为默认大小） */
.text-cloud {
  position: relative;
  font-size: 32px;
  min-height: 40px;
  color: rgba(0, 0, 0, 1);
  font-weight: 400;
  text-shadow: 
    /* 上下左右 4px 实心 */
    -4px -4px 0 #fff,  4px -4px 0 #fff,
    -4px  4px 0 #fff,  4px  4px 0 #fff,
    -4px  0   0 #fff,  4px  0   0 #fff,
     0   -4px 0 #fff,  0    4px 0 #fff,
    /* 增加 2px 的圆角过渡，让边缘变圆润 */
    -3px -5px 1px #fff,  3px -5px 1px #fff,
    -5px -3px 1px #fff,  5px -3px 1px #fff,
    -5px  3px 1px #fff,  5px  3px 1px #fff,
    -3px  5px 1px #fff,  3px  5px 1px #fff;
}
.svg-text {
  display: block;
  overflow: visible; /* 防止描边被裁切 */
}
.svg-text text {
  font-size: 32px;
  font-weight: 400;
  
  fill: #000;               /* 文字填充黑色 */
  stroke: #fff;             /* 描边白色 */
  stroke-width: 15px;        /* 描边宽度（可以随意改大） */
  stroke-linejoin: round;   /* 关键：让转角变成圆角，绝对无棱角 */
  stroke-linecap: round;    /* 让线条两端是圆头 */

  /* 关键：先画描边，再画填充，防止白边吃掉了中间的黑色文字 */
  paint-order: stroke fill; 
}
.primary {
    background-color: #ffd900; color: rgb(0, 0, 0);
}
</style>