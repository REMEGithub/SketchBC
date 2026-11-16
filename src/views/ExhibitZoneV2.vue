<template>
    <main class="drawing-page" @mousemove="handleMouseMove">
        <!-- 图片展示区域 -->
        <canvas ref="canvasRef" class="image-canvas"></canvas>

        <!-- 右上角倒计时：悬浮于图片上方 -->
        <div v-if="showTimer" class="countdown-panel">
            <span class="timer-label">NEXT IMAGE</span>
            <span class="timer-value">{{ formattedTime }}</span>
        </div>

        <!-- 底部悬浮功能栏 -->
        <Transition name="toolbar">
            <div v-if="toolbarVisible" class="function-bar" @mousemove.stop="handleMouseMove">
                <button class="function-btn" @click="prevImage">
                    <!-- 上一张 -->
                    <svg width="44" height="44" viewBox="0 0 44 44" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <g transform="scale(0.366667)">
                            <path d="M78.5 25L43 60.5L78.5 96" stroke="white" stroke-opacity="0.8" stroke-width="8" />
                        </g>
                    </svg>
                </button>

                <button class="function-btn" @click="togglePause">
                    <!-- 暂停 -->
                     <svg v-if="isPaused" width="44" height="44" viewBox="0 0 44 44" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <g transform="scale(0.366667)">
                            <path d="M43 25L94 60L43 95V25Z" stroke="white" stroke-opacity="0.8" stroke-width="8"
                                stroke-linejoin="round" />
                        </g>
                    </svg>
                    <svg v-else width="44" height="44" viewBox="0 0 44 44" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <g transform="scale(0.366667)">
                            <path d="M43 25V94" stroke="white" stroke-opacity="0.8" stroke-width="8" />
                            <path d="M76.5 25V94" stroke="white" stroke-opacity="0.8" stroke-width="8" />
                        </g>
                    </svg>
                    
                </button>

                <button class="function-btn" @click="nextImage">
                    <!-- 下一张 -->
                    <svg width="44" height="44" viewBox="0 0 44 44" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <g transform="scale(0.366667)">
                            <path d="M44 24L79.5 59.5L44 95" stroke="white" stroke-opacity="0.8" stroke-width="8" />
                        </g>
                    </svg>
                </button>

                <button class="function-btn" :class="{ active: isAlwaysOnTop }" @click="toggleAlwaysOnTop">
                    <svg width="44" height="44" viewBox="0 0 44 44" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path
                            d="M39.3938 16.3627L27.7149 4.70098C27.4055 4.3916 27.0059 4.24121 26.602 4.24121C26.1981 4.24121 25.7942 4.3916 25.4891 4.70098L17.8535 12.3279C17.2735 12.2635 16.6934 12.2334 16.109 12.2334C12.6414 12.2334 9.16954 13.3721 6.32072 15.6537C5.59025 16.2338 5.53009 17.3295 6.19181 17.9912L14.8027 26.5893L4.59337 36.7814C4.46876 36.9061 4.39142 37.0693 4.37423 37.2455L4.21525 39.0072C4.17228 39.4541 4.52892 39.8322 4.9672 39.8322C4.99298 39.8322 5.01447 39.8322 5.04025 39.8279L6.80197 39.6689C6.97814 39.6561 7.14142 39.5744 7.26603 39.4498L17.4754 29.2533L26.0863 37.8514C26.3957 38.1607 26.7953 38.3111 27.1992 38.3111C27.659 38.3111 28.1145 38.1135 28.4281 37.7225C31.0965 34.3967 32.2051 30.2287 31.7539 26.2025L39.3895 18.5799C40.0082 17.9697 40.0082 16.9771 39.3938 16.3627ZM29.3434 23.8006L28.1832 24.9607L28.3637 26.5893C28.5399 28.1834 28.4109 29.7689 27.977 31.3072C27.7192 32.2053 27.3711 33.0646 26.9242 33.8725L10.1836 17.1533C10.7938 16.8182 11.4297 16.5346 12.0957 16.3068C13.3848 15.86 14.734 15.6408 16.109 15.6408C16.5645 15.6408 17.0242 15.6623 17.4797 15.7182L19.1082 15.8986L26.6063 8.40918L35.6813 17.4713L29.3434 23.8006Z"
                            fill="white" />
                    </svg>
                </button>

                <button class="function-btn" :class="{ active: isGrayscale }" @click="toggleGrayscale">
                    <svg width="44" height="44" viewBox="0 0 44 44" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path
                            d="M34.0208 9.97918C37.2089 13.1673 39 17.4913 39 22C39 26.5087 37.2089 30.8327 34.0208 34.0208C30.8327 37.2089 26.5087 39 22 39C17.4913 39 13.1673 37.2089 9.97918 34.0208C6.79107 30.8327 5 26.5087 5 22C5 17.4913 6.79107 13.1673 9.97918 9.97918C13.1673 6.79107 17.4913 5 22 5C26.5087 5 30.8327 6.79107 34.0208 9.97918ZM31.4831 12.5169C28.968 10.0018 25.5568 8.58889 22 8.58889C18.4432 8.58889 15.032 10.0018 12.5169 12.5169C10.0018 15.032 8.58889 18.4432 8.58889 22C8.58889 25.5568 10.0018 28.968 12.5169 31.4831L31.4831 12.5169Z"
                            fill="white" />
                    </svg>
                </button>

                <button class="function-btn" :class="{ active: showTimer }" @click="showTimer = !showTimer">
                    <svg width="44" height="44" viewBox="0 0 44 44" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path
                            d="M16.7243 5.95H26.8758C27.3996 5.95 27.9019 5.74192 28.2723 5.37154C28.6427 5.00115 28.8508 4.49881 28.8508 3.975C28.8508 3.4512 28.6427 2.94885 28.2723 2.57846C27.9019 2.20808 27.3996 2 26.8758 2H16.7243C16.2005 2 15.6981 2.20808 15.3277 2.57846C14.9573 2.94885 14.7493 3.4512 14.7493 3.975C14.7493 4.49881 14.9573 5.00115 15.3277 5.37154C15.6981 5.74192 16.2005 5.95 16.7243 5.95ZM21.8 8.90001C18.6751 8.90001 15.6203 9.82666 13.022 11.5628C10.4237 13.2989 8.39858 15.7665 7.20271 18.6536C6.00685 21.5407 5.69395 24.7176 6.3036 27.7825C6.91325 30.8474 8.41805 33.6627 10.6277 35.8723C12.8374 38.082 15.6527 39.5868 18.7176 40.1964C21.7825 40.8061 24.9594 40.4932 27.8464 39.2973C30.7335 38.1015 33.2011 36.0763 34.9373 33.478C36.6734 30.8797 37.6 27.825 37.6 24.7C37.6 20.5096 35.9354 16.4908 32.9723 13.5277C30.0092 10.5646 25.9904 8.90001 21.8 8.90001ZM21.8 36.55C19.4563 36.55 17.1652 35.855 15.2165 34.5529C13.2678 33.2509 11.7489 31.4001 10.852 29.2348C9.95514 27.0695 9.72047 24.6869 10.1777 22.3882C10.6349 20.0895 11.7635 17.9781 13.4208 16.3208C15.0781 14.6635 17.1895 13.5349 19.4882 13.0777C21.7869 12.6205 24.1695 12.8551 26.3348 13.752C28.5001 14.6489 30.3509 16.1678 31.6529 18.1165C32.955 20.0652 33.65 22.3563 33.65 24.7C33.65 26.2562 33.3435 27.7971 32.748 29.2348C32.1525 30.6725 31.2796 31.9789 30.1792 33.0792C29.0789 34.1796 27.7725 35.0525 26.3348 35.648C24.8971 36.2435 23.3562 36.55 21.8 36.55Z"
                            fill="white" />
                        <path
                            d="M27.883 23.7251H23.775V18.0766C23.775 17.5528 23.5669 17.0504 23.1966 16.68C22.8262 16.3096 22.3238 16.1016 21.8 16.1016C21.2762 16.1016 20.7739 16.3096 20.4035 16.68C20.0331 17.0504 19.825 17.5528 19.825 18.0766V25.7001C19.825 26.2239 20.0331 26.7262 20.4035 27.0966C20.7739 27.467 21.2762 27.6751 21.8 27.6751H27.883C28.4068 27.6751 28.9092 27.467 29.2796 27.0966C29.6499 26.7262 29.858 26.2239 29.858 25.7001C29.858 25.1763 29.6499 24.6739 29.2796 24.3035C28.9092 23.9332 28.4068 23.7251 27.883 23.7251Z"
                            fill="white" />
                    </svg>
                </button>

                <button class="function-btn exit-btn" @click="exitToHome">
                    <svg width="44" height="44" viewBox="0 0 44 44" fill="none" xmlns="http://www.w3.org/2000/svg">
                        <path
                            d="M16 7.50016C22.6667 7.33349 36 7.10016 36 7.50016C36 7.87419 36 24.8903 36 35.0016C36 36.1062 35.1046 37 34 37H16"
                            stroke="white" stroke-width="4" stroke-linecap="round" />
                        <path
                            d="M24.9999 22.4995C24.5999 22.3923 11.6667 22.4548 7 22.4995M7 22.4995L11.5 28M7 22.4995L11 16.5"
                            stroke="white" stroke-width="4" stroke-linecap="round" />
                    </svg>
                </button>
            </div>
        </Transition>
    </main>
</template>

<script setup lang="ts">
import {
    ref,
    computed,
    onMounted,
    onUnmounted
} from 'vue'

import { invoke } from '@tauri-apps/api/core'
import {
    getCurrentWindow,
    LogicalSize
} from '@tauri-apps/api/window'

import { useRouter } from 'vue-router'
import { useDrawingParamStore } from '@/stores/useDrawingParamStore'
import { useTimerStore } from '@/stores/useTimeStore'

// ==============================
// 基础数据
// ==============================

const router = useRouter()
const Dstore = useDrawingParamStore()
const Tstore = useTimerStore()

const canvasRef = ref<HTMLCanvasElement | null>(null)

// 图片总数由 Rust 返回
const imageCount = ref(0)

// 当前图片索引
const currentIndex = ref(0)

// 当前图片对象
const currentImage = ref<HTMLImageElement | null>(null)

// ==============================
// 页面状态
// ==============================

// 倒计时
const remainingSeconds = ref(30)

// 暂停状态
const isPaused = ref(false)

// 倒计时显隐
const showTimer = ref(true)

// 黑白模式
const isGrayscale = ref(false)

// 窗口置顶状态
const isAlwaysOnTop = ref(false)

// 工具栏显隐
const toolbarVisible = ref(true)

// ==============================
// 定时器
// ==============================

let countdownInterval:
    ReturnType<typeof setInterval> | null = null

let toolbarTimeout:
    ReturnType<typeof setTimeout> | null = null

let unlistenResize: (() => void) | null = null

// 从 Pinia 获取展示时间
const displaySeconds = computed(() => {
    return Math.max(
        1,
        Number(Dstore.displaySeconds) || 1
    )
})

// 格式化倒计时
const formattedTime = computed(() => {
    const minutes = Math.floor(remainingSeconds.value / 60)
    const seconds = remainingSeconds.value % 60

    return `${String(minutes).padStart(2, '0')}:${String(seconds).padStart(2, '0')}`
})

// 清除倒计时
function clearCountdown() {
    if (countdownInterval !== null) {
        clearInterval(countdownInterval)
        countdownInterval = null
    }
}

// 重置倒计时
function resetCountdown() {
    clearCountdown()

    remainingSeconds.value = displaySeconds.value

    if (!isPaused.value) {
        startCountdown()
    }
}

// 开始倒计时
function startCountdown() {
    clearCountdown()

    if (isPaused.value) return

    countdownInterval = setInterval(() => {
        if (remainingSeconds.value > 0) {
            remainingSeconds.value--
        }

        if (remainingSeconds.value <= 0) {
            nextImage()
        }
    }, 1000)
}

// 暂停 / 继续
function togglePause() {
    isPaused.value = !isPaused.value

    if (isPaused.value) {
        clearCountdown()
    } else {
        startCountdown()
    }
}

// ==============================
// 图片读取和绘制
// ==============================

interface RustImageData {
    mime_type: string
    data_base64: string
}

// 绘制当前图片
function drawCurrentImage() {
    const canvas = canvasRef.value
    const img = currentImage.value

    if (!canvas || !img) return

    const ctx = canvas.getContext('2d')
    if (!ctx) return

    // 根据 Canvas 实际分辨率绘制图片
    ctx.clearRect(0, 0, canvas.width, canvas.height)

    ctx.filter = isGrayscale.value
        ? 'grayscale(100%)'
        : 'none'

    ctx.drawImage(
        img,
        0,
        0,
        canvas.width,
        canvas.height
    )
}

// 从 Rust 读取指定图片
async function loadImage(index: number) {
    if (imageCount.value <= 0) return

    try {
        const result = await invoke<RustImageData>(
            'read_image',
            { index }
        )

        const img = new Image()

        img.onload = () => {
            currentImage.value = img
            resizeCanvas()
        }

        img.onerror = () => {
            console.error('图片加载失败:', index)
        }

        img.src =
            `data:${result.mime_type};base64,${result.data_base64}`

    } catch (error) {
        console.error('读取图片失败:', error)
    }
}

// 重新适配 Canvas 分辨率
function resizeCanvas() {
    const canvas = canvasRef.value
    const img = currentImage.value

    if (!canvas || !img) return

    const width = window.innerWidth
    const height = window.innerHeight

    const dpr = window.devicePixelRatio || 1

    // 计算适应窗口的图片显示尺寸
    const scale = Math.min(
        width / img.naturalWidth,
        height / img.naturalHeight
    )

    const displayWidth = img.naturalWidth * scale
    const displayHeight = img.naturalHeight * scale

    // Canvas 实际绘制分辨率
    canvas.width = Math.round(displayWidth * dpr)
    canvas.height = Math.round(displayHeight * dpr)

    // CSS 显示尺寸
    canvas.style.width = `${displayWidth}px`
    canvas.style.height = `${displayHeight}px`

    // 重新绘制图片
    drawCurrentImage()
}

// ==============================
// 图片切换
// ==============================

// 上一张
async function prevImage() {
    if (imageCount.value <= 0) return

    currentIndex.value =
        (currentIndex.value - 1 + imageCount.value)
        % imageCount.value

    await loadImage(currentIndex.value)

    resetCountdown()
}

// 下一张
async function nextImage() {
    if (imageCount.value <= 0) return

    currentIndex.value =
        (currentIndex.value + 1) % imageCount.value

    await loadImage(currentIndex.value)

    resetCountdown()
}

// ==============================
// 黑白模式
// ==============================

function toggleGrayscale() {
    isGrayscale.value = !isGrayscale.value

    // 不重新读取图片，直接重新绘制 Canvas
    drawCurrentImage()
}

// ==============================
// 窗口置顶
// ==============================

async function toggleAlwaysOnTop() {
    try {
        isAlwaysOnTop.value = !isAlwaysOnTop.value

        await getCurrentWindow().setAlwaysOnTop(
            isAlwaysOnTop.value
        )
    } catch (error) {
        isAlwaysOnTop.value = !isAlwaysOnTop.value
        console.error('设置窗口置顶失败:', error)
    }
}

// ==============================
// 工具栏自动显隐
// ==============================

function handleMouseMove() {
    toolbarVisible.value = true

    if (toolbarTimeout !== null) {
        clearTimeout(toolbarTimeout)
    }

    toolbarTimeout = setTimeout(() => {
        toolbarVisible.value = false
    }, 5000)
}

// ==============================
// 返回主页
// ==============================

async function exitToHome() {
    clearCountdown()

    try {
        // 取消窗口置顶
        if (isAlwaysOnTop.value) {
            await getCurrentWindow().setAlwaysOnTop(false)
            isAlwaysOnTop.value = false
        }

        // 清除 Rust 中的文件夹路径和图片列表
        await invoke('clear_image_folder')

        // 清除 Pinia 数据
        Dstore.imageCount = 0
        Dstore.folderName = ''
        Dstore.displaySeconds = 0
        Tstore.setTime(0)

        // 恢复主页窗口尺寸和不可调整状态
        const win = getCurrentWindow()

        await win.setSize(new LogicalSize(800, 800))
        await win.setResizable(false)

        // 返回主页
        await router.push({ name: 'HomePage' })

    } catch (error) {
        console.error('返回主页失败:', error)
    }
}

// ==============================
// 生命周期
// ==============================

onMounted(async () => {
    try {
        const win = getCurrentWindow()

        // 展示页面允许调整窗口大小
        await win.setResizable(true)

        // 从 Rust 获取图片数量
        imageCount.value = await invoke<number>(
            'get_image_count'
        )

        if (imageCount.value <= 0) {
            console.warn('没有可展示的图片')
            return
        }

        // 初始化倒计时
        remainingSeconds.value = displaySeconds.value

        // 读取第一张图片
        await loadImage(currentIndex.value)

        // 监听窗口尺寸变化
        unlistenResize = await getCurrentWindow().onResized(() => {
            resizeCanvas()
        })

        // 开始倒计时
        startCountdown()

        // 初始化工具栏隐藏计时
        handleMouseMove()

    } catch (error) {
        console.error('初始化展示页面失败:', error)
    }
})

onUnmounted(() => {
    unlistenResize?.()
    unlistenResize = null

    clearCountdown()


    if (toolbarTimeout !== null) {
        clearTimeout(toolbarTimeout)
    }
})
</script>


<style scoped>
.drawing-page {
    position: relative;

    width: 100%;
    height: 100vh;

    /* 不使用 flex 布局挤压图片 */
    overflow: hidden;

    background: #111;
    user-select: none;
}

/* 图片始终占据整个展示区域 */
.image-canvas {
    position: absolute;

    left: 50%;
    top: 50%;

    transform: translate(-50%, -50%);

    display: block;

    max-width: 100%;
    max-height: 100%;

    object-fit: contain;
}

/* 右上角倒计时 */
.countdown-panel {
    position: absolute;

    top: 24px;
    right: 24px;

    z-index: 20;

    min-width: 105px;
    padding: 12px 18px;

    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 5px;

    color: white;

    background: rgba(30, 30, 30, 0.65);

    border: 1px solid rgba(255, 255, 255, 0.12);
    border-radius: 10px;

    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);

    pointer-events: none;
}

.timer-label {
    font-size: 10px;
    letter-spacing: 1.5px;
    color: rgba(255, 255, 255, 0.65);
}

.timer-value {
    font-size: 28px;
    font-weight: 600;
    font-variant-numeric: tabular-nums;
    letter-spacing: 1px;
}

/* 底部悬浮功能栏 */
.function-bar {
    position: absolute;

    left: 0;
    right: 0;
    bottom: 0;

    z-index: 30;

    height: 60px;

    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;

    background: rgba(35, 35, 35, 0.75);

    backdrop-filter: blur(12px);
    -webkit-backdrop-filter: blur(12px);
}

/* 功能按钮 */
.function-btn {
    width: 45px;
    height: 45px;

    flex-shrink: 0;

    display: flex;
    justify-content: center;
    align-items: center;

    border: 1px solid rgba(255, 255, 255, 0.15);
    border-radius: 8px;

    background: rgba(255, 255, 255, 0.12);

    color: white;
    font-size: 11px;

    cursor: pointer;

    transition:
        background 0.2s,
        border-color 0.2s;
}

.function-btn:hover {
    background: rgba(255, 255, 255, 0.28);
}

.function-btn.active {
    background: rgba(255, 255, 255, 0.3);
    border-color: rgba(255, 255, 255, 0.5);
}

.exit-btn:hover {
    background: rgba(180, 60, 60, 0.7);
}

/* 工具栏过渡动画 */
.toolbar-enter-active,
.toolbar-leave-active {
    transition:
        opacity 0.25s ease,
        transform 0.25s ease;
}

.toolbar-enter-from,
.toolbar-leave-to {
    opacity: 0;
    transform: translateY(15px);
}

.function-btn svg {
    width: 32px;
    height: 32px;
    display: block;
}
</style>
