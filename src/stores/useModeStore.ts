import { defineStore } from "pinia";
import { ref } from "vue";

export const useModeStore = defineStore('mode', ()=>{
    // state
    const currentMode = ref<'quick' | 'custom' | 'random'>('quick');
    
    const setMode = (mode:'quick' | 'custom' | 'random')=>{
        currentMode.value = mode;
    }

    
    return {currentMode, setMode};
})