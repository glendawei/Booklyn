<template>
  <div class="border rounded-lg p-4 mb-4 hover:bg-gray-50 transition">
    <h4 class="font-semibold mb-2">{{ thread.title }}</h4>
    <img v-if="thread.image" :src="thread.image" class="w-full h-32 object-cover mb-2" />
    <p class="text-sm text-gray-600">{{ thread.excerpt }}</p>
    <div class="mt-2 flex justify-between text-xs text-gray-400">
      <span>{{ thread.user }}</span>
      <span>{{ thread.date }}</span>
    </div>
    <router-link :to="`/thread/${thread.id}`" class="text-blue-500 text-sm">看詳情 →</router-link>
  </div>
</template>

<script setup>
defineProps({ thread: Object })
</script>
