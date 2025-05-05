<template>
  <div class="border p-4 rounded-lg flex items-center space-x-4">
    <img :src="user.avatar" class="w-16 h-16 rounded-full" />
    <div>
      <h3 class="text-xl font-medium">{{ user.name }}</h3>
      <p class="text-sm text-gray-500">{{ user.bio }}</p>
      <div class="mt-2 text-sm">
        <span class="mr-4">📚 {{ user.bookCount }}</span>
        <span>💬 {{ user.threadCount }}</span>
      </div>
    </div>
  </div>
</template>

<script setup>
defineProps({ user: Object })
</script>

