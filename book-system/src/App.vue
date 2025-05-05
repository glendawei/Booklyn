<!-- src/App.vue -->
<script setup>
import { RouterView } from 'vue-router'
import NavBar from '@/components/NavBar.vue'
import Footer    from '@/components/Footer.vue'
</script>

<template>
  <!-- 常駐搜尋列 -->
  <NavBar />

  <!-- 動態切換頁面 -->
  <main class="pt-12">
    <RouterView />
  </main>

  <!-- 常駐頁尾 -->

</template>

<style>
/* App 內如果還有全域樣式可以放這 */
</style>
