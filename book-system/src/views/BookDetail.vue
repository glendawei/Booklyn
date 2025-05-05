<template>
  <div class="space-y-6">
    <div class="flex space-x-8">
      <img :src="book.cover" class="w-48 h-64 object-cover"/>
      <div>
        <h2 class="text-2xl font-bold mb-2">{{ book.title }}</h2>
        <p class="text-sm text-gray-500">{{ book.author }}</p>
        <p class="mt-4">{{ book.overview }}</p>
        <button class="mt-4 px-4 py-2 bg-blue-600 text-white rounded">Buy</button>
      </div>
    </div>

    <div>
      <h3 class="font-semibold mb-2">Rating & Review</h3>
      <ReviewForm @posted="loadReviews" />
    </div>

    <div>
      <h3 class="font-semibold mb-2">Community Reviews</h3>
      <div class="space-y-4">
        <ReviewCard v-for="r in reviews" :key="r.id" :review="r"/>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import ReviewForm from '@/components/ReviewForm.vue'
import ReviewCard from '@/components/ReviewCard.vue'
import { useBookStore } from '@/store/book'
import { useRoute } from 'vue-router'

const route = useRoute()
const store = useBookStore()
const book=ref({}), reviews=ref([])

async function load() {
  book.value    = await store.fetchBook(route.params.id)
  reviews.value = await store.fetchBookReviews(route.params.id)
}

onMounted(load)
function loadReviews() {
  load()
}
</script>

