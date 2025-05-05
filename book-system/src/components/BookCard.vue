<template>
  <div class="border rounded-lg overflow-hidden shadow hover:shadow-lg transition p-2">
    <img :src="book.cover" class="w-full h-40 object-cover mb-2" />
    <h3 class="text-lg font-medium">{{ book.title }}</h3>
    <p class="text-sm text-gray-500">{{ book.author }}</p>
  </div>
</template>

<script setup>
defineProps({
  book: Object
})
</script>
