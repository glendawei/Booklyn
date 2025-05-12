<template>
  <div class="book-detail p-6 max-w-3xl mx-auto">
    <div v-if="book">
      <h1 class="text-3xl font-bold mb-4">{{ book.title }}</h1>
      <p class="text-lg text-gray-700 mb-2">Author: {{ book.author }}</p>
      <p class="text-lg text-yellow-600 mb-2">Rating: {{ book.rate }}</p>
      <div class="text-lg text-gray-800 mb-4">
        <strong>Description:</strong> {{ book.description }}
      </div>

      <div v-if="book.reviews && book.reviews.length" class="mt-6">
        <h2 class="text-2xl font-semibold mb-2">Reader Reviews:</h2>
        <ul class="space-y-2">
          <li
            v-for="(review, index) in book.reviews"
            :key="index"
            class="bg-gray-100 p-4 rounded shadow"
          >
            <p>
              <strong>{{ review.user }}</strong> rated it {{ review.rating }}/5
            </p>
            <p class="text-gray-700 italic">"{{ review.comment }}"</p>
          </li>
        </ul>
      </div>
    </div>

    <div v-else>
      <p class="text-red-500">Book not found.</p>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from "vue";
import { useRoute } from "vue-router";
import { bookshelves } from "../data/mockBookshelves.js"; // Adjust path as needed

const route = useRoute();
const bookId = Number(route.params.id);
const book = ref(null);

onMounted(() => {
  // Step 1: Collect all arrays from the bookshelves object
  const allShelves = Object.values(bookshelves);

  // Step 2: Use a Set to avoid duplicate books (in case same book appears in multiple shelves)
  const bookMap = new Map();
  allShelves.flat().forEach((book) => {
    bookMap.set(book.id, book);
  });

  // Step 3: Find the book by id
  book.value = bookMap.get(bookId);
});
</script>

<style scoped>
.book-detail img {
  max-height: 400px;
  object-fit: cover;
}
</style>
