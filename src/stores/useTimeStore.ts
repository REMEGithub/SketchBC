
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { useDrawingParamStore } from './useDrawingParamStore'

export const useTimerStore = defineStore('timer', () => {

    //另一个store的引用
    const picNum = computed(() => useDrawingParamStore().imageCount)

    // 当前选中的时间，单位：秒
    const selectedSeconds = ref(0)

    // 时间选项
    const presets = ref([
        { label: '30s', seconds: 30 },
        { label: '1min', seconds: 60 },
        { label: '2min', seconds: 120 },
        { label: '5min', seconds: 300 },
        { label: '10min', seconds: 600 },
        { label: '30min', seconds: 1800 },
        { label: '45min', seconds: 2700 },
    ])

    // 当前选中的时间标签
    const selectedLabel = computed(() => {
        return presets.value.find(
            item => item.seconds === selectedSeconds.value
        )?.label ?? ''
    })

    // 格式化时间
    const formattedTime = computed(() => {
        const seconds = selectedSeconds.value * picNum.value

        const h = Math.floor(seconds / 3600)
        const m = Math.floor((seconds % 3600) / 60)
        const s = seconds % 60

        return `${String(h).padStart(2, '0')}时${String(m).padStart(2, '0')}分${String(s).padStart(2, '0')}秒`
    })

    // 修改选中时间
    const setTime = (seconds: number) => {
        selectedSeconds.value = seconds;
    }

    return {
        selectedSeconds,
        presets,
        selectedLabel,
        formattedTime,
        setTime
    }
})
