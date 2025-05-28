<!-- src/App.vue -->
<script setup>
import { RouterView } from 'vue-router'
import SearchBar from '@/components/SearchBar.vue'
import Footer    from '@/components/Footer.vue'


</script>

<template>
  <!-- 常駐搜尋列 -->
  <SearchBar />

  <!-- 動態切換頁面 -->
  <main class="main-content">
    <RouterView />
  </main>

  <!-- 常駐頁尾 -->
  <Footer />
</template>

<style>
/* App 內如果還有全域樣式可以放這 */
.main-content {
  margin-top: 0rem;  /* ✅ 往下推 5rem，可視 SearchBar 高度調整 */
  padding: 5rem 3rem 3rem 3rem;
  background-color: #fffdef;
}
</style>