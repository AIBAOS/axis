<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <p>
      欢迎使用 Axis NAS 管理面板
      <br />
      基于 Vue 3 + Vite + TailwindCSS 构建
    </p>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  msg: string
}>()
</script>

<style scoped>
.hello {
  text-align: center;
}

h1 {
  font-weight: 500;
  font-size: 2.6rem;
  position: relative;
  top: -10px;
}

p {
  color: #42b883;
}
</style>
