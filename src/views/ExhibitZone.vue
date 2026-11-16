<template>
  <div>
    <h1>lalala</h1>
    <PageJumpButton label="返回" :to="{name: 'HomePage'}"/>
  </div>
  <div class="card-container" :class="{ 'is-flipped': isFlipped }" @click="toggleFlip">
    <div class="card-inner">
      
      <div class="card-front">
        <div class="card-content">
          <h3>卡牌正面</h3>
          <p>（这里暂时放静态图或文字，以后可以换成 Spine Canvas）</p>
        </div>
      </div>

      <div class="card-back">
        <div class="card-content">
          <h3>卡牌背面</h3>
          <p>点击再次翻转</p>
        </div>
      </div>

    </div>
  </div>

</template>

<script setup lang="ts">
import PageJumpButton from '@/components/common/PageJumpButton.vue';
import { ref } from 'vue';

// 控制卡牌翻转状态的响应式变量
const isFlipped = ref(false);

const toggleFlip = () => {
  isFlipped.value = !isFlipped.value;
};
</script>

<style scoped>
/* 1. 舞台：设置 3D 空间的视距（值越小，透视畸变越强烈） */
.card-container {
  width: 240px;
  height: 340px;
  perspective: 1000px; 
  cursor: pointer;
  margin: 50px auto;
}

/* 2. 容器：承载正反面，并执行旋转动画 */
.card-inner {
  position: relative;
  width: 100%;
  height: 100%;
  transition: transform 0.6s cubic-bezier(0.4, 0, 0.2, 1); /* 平滑的翻转曲线 */
  transform-style: preserve-3d; /* 必须：开启子元素的 3D 渲染 */
}

/* 当激活 is-flipped 类时，沿 Y 轴旋转 180 度 */
.card-container.is-flipped .card-inner {
  transform: rotateY(180deg);
}

/* 3. 正反面共有样式 */
.card-front, .card-back {
  position: absolute;
  width: 100%;
  height: 100%;
  backface-visibility: hidden; /* 必须：当转到背面时隐藏自己，防止穿透 */
  border-radius: 16px;
  border: 2px solid #ccc;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 8px 16px rgba(0, 0, 0, 0.15);
}

/* 4. 正面独有样式 */
.card-front {
  background: linear-gradient(135deg, #e0e0e0, #ffffff);
  color: #333;
}

/* 5. 背面独有样式 */
.card-back {
  background: linear-gradient(135deg, #1e3c72, #2a5298);
  color: white;
  transform: rotateY(180deg); /* 核心：初始状态就将背面水平翻转 180 度 */
}

.card-content {
  text-align: center;
  padding: 20px;
}
</style>