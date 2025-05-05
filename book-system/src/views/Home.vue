<template>
  <section class="grid grid-cols-4 gap-6 p-6">
    <!-- 主內容 3 欄 -->
    <div class="col-span-3 space-y-12">
      <!-- 喜好推薦 -->
      <div>
        <h2 class="text-2xl font-bold mb-4">
          Because You Like {{ userPreferenceLabel }}
        </h2>
        <div class="grid grid-cols-4 gap-4">
          <!-- 用 8 個假資料測試 -->
          <BookCard
            v-for="b in recs"
            :key="b.id"
            :book="b"
          />
        </div>
      </div>

      <!-- 其他推薦 -->
      <div>
        <h2 class="text-2xl font-bold mb-4">Other Recommendations</h2>
        <div class="grid grid-cols-8 gap-2">
          <!-- 用 16 個假資料測試 -->
          <BookCard
            v-for="b in others"
            :key="b.id"
            :book="b"
          />
        </div>
      </div>
    </div>

    <!-- 右邊 News 側欄 1 欄 -->
    <aside class="col-span-1 bg-gray-100 p-4">
      <h2 class="text-xl font-semibold border-b pb-2 mb-4">News</h2>
      <div class="space-y-4">
        <div class="h-24 bg-gray-300 animate-pulse rounded"></div>
        <div class="h-24 bg-gray-300 animate-pulse rounded"></div>
        <div class="h-24 bg-gray-300 animate-pulse rounded"></div>
      </div>
    </aside>
  </section>
</template>

<script setup>
import { ref } from 'vue'
import BookCard from '@/components/BookCard.vue'

// 假資料，只要有 id，BookCard 裡面展示大頭照、標題等都會走預設 placeholder
const userPreferenceLabel = ref('Romance')
const recs = ref(
  Array.from({ length: 8 }, (_, i) => ({
    id: `rec-${i}`,
    title: '',
    coverUrl: ''  // BookCard 若沒圖可自行顯示 default
  }))
)
const others = ref(
  Array.from({ length: 16 }, (_, i) => ({
    id: `other-${i}`,
    title: '',
    coverUrl: ''
  }))
)
</script>

<style scoped>
/* 若 BookCard 高度需要固定，或想讓 grid-auto-rows 自動調整，可在這裡 tweak */
</style>
