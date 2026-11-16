<template>
    <button @click="handleBack">
        返回
    </button>
</template>

<script setup lang="ts">
    import { useRouter } from 'vue-router'

    import { getCurrentWindow } from '@tauri-apps/api/window'
    import { LogicalSize } from '@tauri-apps/api/dpi'

    import { invoke } from '@tauri-apps/api/core'

    import { useDrawingParamStore } from '@/stores/useDrawingParamStore'
    import { useTimerStore } from '@/stores/useTimeStore'

    const router = useRouter()
    const Dstore = useDrawingParamStore()
    const Tstore = useTimerStore()

    const handleBack = async () => {
        try {

            // 清除 Rust 中的文件夹路径和图片列表
            await invoke('clear_image_folder')

            // 清除 Pinia 数据
            Dstore.imageCount = 0
            Dstore.folderName = ''
            Dstore.displaySeconds = 0
            Tstore.setTime(0)

            // 恢复窗口尺寸为 800 × 800
            const win = getCurrentWindow()
            await win.setSize(new LogicalSize(800, 800))

            // 重新锁定窗口大小
            await getCurrentWindow().setResizable(false)

            // 返回主页面
            await router.push({ name: 'HomePage' })

        } catch (error) {
            console.error('返回主页面失败:', error)
        }
    }
</script>

<style scoped></style>