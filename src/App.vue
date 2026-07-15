<template>
  <div id="app">
    
    <RouterView/>
  </div>
</template>

<script setup lang="ts">
</script>

<style>
html, body, #app {  /* 加上html和body防止原生浏览器的8px margin */
  padding: 0;
  margin: 0;
  width: 100%;
  height: 100vh;
  overflow: hidden;
}
</style>