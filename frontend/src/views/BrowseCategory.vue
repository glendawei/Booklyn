<template>
  <div class="book-search-page">
    <aside class="filters">
      <h2>Genres</h2>
      <div v-for="(books, genre) in bookshelves" :key="genre">
        <label>
          <input type="checkbox" :value="genre" v-model="selectedGenres" />
          {{ genre }}
        </label>
      </div>

      <h2>Authors</h2>
      <input
        type="text"
        v-model="authorSearch"
        placeholder="Search authors..."
        class="author-search"
      />
      <div v-if="authorSearch">
        <div v-for="author in filteredAuthors" :key="author">
          <label>
            <input type="checkbox" :value="author" v-model="selectedAuthors" />
            {{ author }}
          </label>
        </div>
      </div>

      <h2>Minimum Avg Rating</h2>
      <input type="range" min="0" max="5" step="0.1" v-model="minAvgRating" />
      <span style="margin-left: 15px">{{ minAvgRating }}</span>
    </aside>
    <main class="book-results">
      <h1 class="page-title">Books</h1>
      <p v-if="searchQuery" style="margin-bottom: 5px">
        Showing results for "{{ searchQuery }}"
      </p>
      <div class="book-grid">
        <BookCard v-for="book in filteredBooks" :key="book.id" :book="book" />
        <p v-if="filteredBooks.length === 0">
          No books match the current filters.
        </p>
      </div>
    </main>
  </div>
</template>

<script setup>
import { ref, computed, watch } from "vue";
import { bookshelves } from "../data/mockBookshelves.js";
import BookCard from "../components/BrowsingBookCard.vue";

import { useRoute } from "vue-router";

const route = useRoute();
const searchQuery = ref(route.query.q || "");

// Watch for route changes in case user navigates again with new search
watch(
  () => route.query.q,
  (newQ) => {
    searchQuery.value = newQ || "";
  }
);

const selectedGenres = ref(["All"]); // Default to All selected
const selectedAuthors = ref([]);
const minAvgRating = ref(0);
const authorSearch = ref("");

// Flatten all books for filtering
const allBooks = computed(() => Object.values(bookshelves).flat());

// Unique authors across all books, sorted
const uniqueAuthors = computed(() => {
  const authorsSet = new Set(allBooks.value.map((book) => book.author));
  return Array.from(authorsSet).sort();
});

// Filter authors based on authorSearch input
const filteredAuthors = computed(() => {
  const search = authorSearch.value.toLowerCase();
  return uniqueAuthors.value.filter((author) =>
    author.toLowerCase().includes(search)
  );
});

// Books filtered by selected genres, authors, and minimum rating
const filteredBooks = computed(() => {
  // If "All" genre selected or no genres selected, consider all genres
  const genresToInclude =
    selectedGenres.value.includes("All") || selectedGenres.value.length === 0
      ? Object.keys(bookshelves)
      : selectedGenres.value;

  // Collect books from selected genres
  const booksToFilter = genresToInclude.flatMap(
    (genre) => bookshelves[genre] || []
  );

  const seenIds = new Set();

  return booksToFilter.filter((book) => {
    const reviews = Array.isArray(book.reviews) ? book.reviews : [];

    const avgRating =
      reviews.length > 0
        ? reviews.reduce((acc, r) => acc + r.rating, 0) / reviews.length
        : 0;

    const matchesAuthor =
      selectedAuthors.value.length === 0 ||
      selectedAuthors.value.includes(book.author);

    const matchesSearch =
      !searchQuery.value ||
      book.title.toLowerCase().includes(searchQuery.value.toLowerCase());

    const isMatch =
      avgRating >= minAvgRating.value && matchesAuthor && matchesSearch;

    const isDuplicate = seenIds.has(book.id);
    if (isMatch && !isDuplicate) {
      seenIds.add(book.id);
      return true;
    }
    return false;
  });
});
</script>

<style scoped>
.book-search-page {
  display: flex;
  gap: 2rem;
  padding: 2rem;
}

.filters {
  width: 250px;
  border-right: 1px solid #ccc;
  padding-right: 20px;
}

.filters h2 {
  margin-top: 1rem;
}

.author-search {
  width: 100%;
  padding: 0.3rem 0.5rem;
  margin-bottom: 0.8rem;
  box-sizing: border-box;
  border: 1px solid #ccc;
  border-radius: 4px;
}

.book-results {
  flex: 1;
}

.page-title {
  font-size: 1.8rem;
  font-weight: 600;
  margin-bottom: 1.5rem;
}

.book-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
  gap: 1.5rem;
}

/* Hide the native checkbox */
input[type="checkbox"] {
  -webkit-appearance: none;
  -moz-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  border: 2px solid #888;
  border-radius: 4px;
  position: relative;
  cursor: pointer;
  outline: none;
}

/* When checked, show a custom checkmark */
input[type="checkbox"]:checked {
  background-color: #d97706; /* box bg color when checked */
  border: 2px solid #88888800;
}

/* Create the checkmark with ::after */
input[type="checkbox"]:checked::after {
  content: "";
  position: absolute;
  left: 5px;
  top: 1px;
  width: 5px;
  height: 10px;
  border: solid white; /* checkmark color */
  border-width: 0 2px 2px 0;
  transform: rotate(45deg);
}
input[type="range"] {
  -webkit-appearance: none;
  width: 150px;
  height: 6px;
  background: #ddd; /* track background */
  border-radius: 3px;
  outline: none;
  margin-top: 0.5rem;
}

/* Track - WebKit */
input[type="range"]::-webkit-slider-runnable-track {
  background: #ddd;
  border-radius: 3px;
  height: 6px;
}

/* Thumb - WebKit */
input[type="range"]::-webkit-slider-thumb {
  -webkit-appearance: none;
  appearance: none;
  width: 18px;
  height: 18px;
  background: #d97706; /* thumb color */
  cursor: pointer;
  border-radius: 50%;
  margin-top: -6px; /* center thumb vertically */
  transition: background-color 0.3s ease;
}

/* Thumb hover */
input[type="range"]::-webkit-slider-thumb:hover {
  background: #b36205;
}

/* Track - Firefox */
input[type="range"]::-moz-range-track {
  background: #ddd;
  height: 6px;
  width: 100px;
  border-radius: 3px;
}

/* Thumb - Firefox */
input[type="range"]::-moz-range-thumb {
  width: 18px;
  height: 18px;
  background: #d97706;
  cursor: pointer;
  border-radius: 50%;
  transition: background-color 0.3s ease;
}

input[type="range"]::-moz-range-thumb:hover {
  background: #b36205;
}
</style>