<template>
  <div class="min-h-screen bg-gray-100">
    <nav class="bg-primary-700 text-white shadow-lg">
      <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div class="flex items-center justify-between h-16">
          <div class="flex items-center">
            <div class="flex-shrink-0">
              <h1 class="text-xl font-bold">Axis NAS 管理面板</h1>
            </div>
          </div>
          <div class="flex items-center space-x-4">
            <span class="text-sm text-primary-200">v0.1.0</span>
          </div>
        </div>
      </div>
    </nav>

    <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
// App root component
</script>

<style scoped>
/* App specific styles */
</style>
