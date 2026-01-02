<!-- src/components/EditorContainer.vue -->
<template>
  <div class="editor-container">
    <!-- 顶部工具栏 -->
    <div class="toolbar">
      <button @click="handleNew">新建</button>
      <button @click="handleOpen">打开</button>
      <button @click="handleSave">保存</button>
      <button @click="handleUndo">撤销</button>
      <button @click="handleRedo">重做</button>
    </div>
    
    <!-- 编辑器区域 -->
    <div class="editor-wrapper">
      <textarea 
        ref="textareaRef"
        v-model="content"
        @input="handleInput"
        @keydown="handleKeydown"
        spellcheck="false"
        class="editor-textarea"
      ></textarea>
      
      <!-- 简单的状态栏 -->
      <div class="status-bar">
        <span>行数: {{ lineCount }}</span>
        <span>字符数: {{ charCount }}</span>
        <span>语言: {{ language }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open, save } from '@tauri-apps/plugin-dialog'

const content = ref('// 欢迎使用编辑器\nconsole.log("Hello, Editor!");\n\nfunction hello() {\n  return "World";\n}')
const textareaRef = ref<HTMLTextAreaElement>()

// 基础状态
const lineCount = computed(() => content.value.split('\n').length)
const charCount = computed(() => content.value.length)
const language = ref('javascript')

// 编辑器操作
const handleInput = () => {
  console.log('内容已修改')
}

const handleKeydown = (e: KeyboardEvent) => {
  // 简单的快捷键支持
  if (e.ctrlKey || e.metaKey) {
    switch(e.key) {
      case 's':
        e.preventDefault()
        handleSave()
        break
      case 'o':
        e.preventDefault()
        handleOpen()
        break
    }
  }
}

// 文件操作
const handleNew = async () => {
  content.value = ''
}

const handleOpen = async () => {
  try {
    const selected = await open({
        multiple: false,
        directory: false,
        filters: [{
        name: "Text Files",
        extensions: ["txt", "md", "js", "vue", "ts", "c", "cpp", ]
        }]
    })
    if (selected) {
      const fileContent = await invoke('read_file', { path: selected })
      content.value = fileContent as string
    }
  } catch (error) {
    content.value = error as string
  }
}

const handleSave = async () => {
  try {
    const selected = await save({
        
    })
    const saved = await invoke('save_file_dialog', { path: selected, content: content.value })
    console.log('保存成功:', saved)
  } catch (error) {
    console.error('保存失败:', error)
  }
}

const handleUndo = () => {
  // TODO: 实现撤销逻辑
}

const handleRedo = () => {
  // TODO: 实现重做逻辑
}

onMounted(() => {
  textareaRef.value?.focus()
})
</script>

<style scoped>
.editor-container {
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.toolbar {
  padding: 8px;
  background: #f5f5f5;
  border-bottom: 1px solid #ddd;
  display: flex;
  gap: 8px;
}

.toolbar button {
  padding: 6px 12px;
  border: 1px solid #ccc;
  background: white;
  border-radius: 4px;
  cursor: pointer;
}

.toolbar button:hover {
  background: #e9e9e9;
}

.editor-wrapper {
  flex: 1;
  position: relative;
}

.editor-textarea {
  width: 100%;
  height: calc(100% - 30px);
  border: none;
  padding: 10px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
  line-height: 1.5;
  resize: none;
  outline: none;
  tab-size: 2;
}

.status-bar {
  height: 30px;
  background: #f5f5f5;
  border-top: 1px solid #ddd;
  padding: 0 10px;
  display: flex;
  align-items: center;
  gap: 20px;
  font-size: 12px;
  color: #666;
}
</style>