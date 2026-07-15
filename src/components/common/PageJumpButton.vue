<template>
  <div class="start-section">
    <Button class = base-btn
    :class = "[variant,size]"
    :disabled = "disabled"
    @click="handleClick"
    >{{ props.label }}</Button>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
const props = defineProps<{
    label: string;                // 按钮文字
    to?: string | object;         // 跳转路由（字符串或路由对象）
    variant?: 'primary' | 'secondary' | 'outline' | 'danger';  // 按钮样式
    size?: 'small' | 'medium' | 'large';                       // 按钮大小
    fullWidth?: boolean;          // 是否占满宽度
    disabled?: boolean           // 是否禁用
}>();

const emit = defineEmits<{
    (e:'click',event:MouseEvent):void
}>();

const router = useRouter();

const handleClick = (event:MouseEvent)=>{
    if(props.disabled) return;

    if(props.to){
        router.push(props.to);
    }else{
        emit('click',event)
    }
}
</script>

<style scoped>
.start-section {
  margin-top: auto;
}

.base-btn {
  font-weight: 500;
  background: #007aff;
  color: white;
  border: none;
  border-radius: 10px;
  cursor: pointer;
}
/* 默认 padding（medium 作为默认大小） */
.base-btn {
  padding: 18px 120px;     /* 默认 padding */
  font-size: 22px;
  min-height: 40px;
}
.primary {
    background-color: #ffd900; color: rgb(0, 0, 0);
}
</style>