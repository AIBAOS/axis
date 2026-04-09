<template>
  <Transition
    enter-active-class="transition duration-200 ease-out"
    enter-from-class="transform translate-y-2 opacity-0"
    enter-to-class="transform translate-y-0 opacity-100"
    leave-active-class="transition duration-150 ease-in"
    leave-from-class="transform translate-y-0 opacity-100"
    leave-to-class="transform translate-y-2 opacity-0"
  >
    <slot />
  </Transition>
</template>

<script setup>
// 页面过渡动画组件
// 使用方式：
// <PageTransition>
//   <router-view />
// </PageTransition>
</script>
