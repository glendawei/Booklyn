<template>
  <div class="flex">
    <aside class="w-64 pr-6">
      <ul class="space-y-2 text-sm text-gray-600">
        <li v-for="c in categories" :key="c">{{ c }}</li>
      </ul>
    </aside>
    <div class="flex-1">
      <ThreadCard v-for="t in threads" :key="t.id" :thread="t"/>
    </div>
  </div>
</template>

<script setup>
import ThreadCard from '@/components/ThreadCard.vue'
import { onMounted, ref } from 'vue'
import { useThreadStore } from '@/store/thread'

const store = useThreadStore()
const threads = ref([]), categories = ['All','Romance','Sci-Fi','Mystery']

onMounted(async () => {
  threads.value = await store.fetchThreads()
})
</script>

