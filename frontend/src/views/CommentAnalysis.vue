<template>
  <div class="max-w-xl mx-auto">
    <button @click="$router.back()" class="mb-4 text-blue-500">← Back</button>
    <h2 class="text-2xl font-bold mb-4">Comment Analysis</h2>
    <canvas ref="chartRef"></canvas>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue'
import Chart from 'chart.js/auto'
import { useRoute } from 'vue-router'
import { useThreadStore } from '@/store/thread'

const route = useRoute()
const store = useThreadStore()
const chartRef = ref(null)

onMounted(async () => {
  const data = await store.fetchAnalysis(route.params.id)
  new Chart(chartRef.value, {
    type: 'pie',
    data: {
      labels: data.labels,
      datasets: [{ data: data.values }]
    }
  })
})
</script>

