
import { defineStore } from 'pinia';

import { ref, computed } from 'vue';

export const useDrawingParamStore = defineStore(
  'drawing',
  () => {

    // 文件夹信息
    const folderName = ref('');
    const imageCount = ref(0);

    // 单张图片展示时间，单位：秒
    const displaySeconds = ref(0);

    const isTimeLocked = ref(true);

    // 文件夹信息更新
    const setFolderInfo = (
      name: string,
      count: number
    ) => {
      folderName.value = name;
      imageCount.value = count;
    };

    const setdisplaySeconds = (seconds: number) => {
      displaySeconds.value = seconds;
    }

    const setIsTimeLocked = (locked: boolean) => {
      isTimeLocked.value = locked;
    }

    // 总展示时间
    const totalSeconds = computed(() => {
      return imageCount.value * displaySeconds.value;
    });

    return {
      folderName,
      imageCount,
      displaySeconds,
      isTimeLocked,
      totalSeconds,
      setFolderInfo,
      setdisplaySeconds,
      setIsTimeLocked
    };
  }
);
