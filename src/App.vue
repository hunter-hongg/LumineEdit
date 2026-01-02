<!-- src/App.vue -->
<template>
  <div id="app">
    <EditorContainer />
  </div>
</template>

<script setup lang="ts">
// 导入组件
import EditorContainer from './components/EditorContainer.vue'
</script>

<style>
#app {
  width: 100vw;
  height: 100vh;
  margin: 0;
  padding: 0;
  overflow: hidden;
}
</style>