<template>
  <div class="book-detail">
    <div v-if="book">
      <div style="display: grid; grid-template-columns: 200px 1fr; column-gap: 50px; margin-bottom: 25px;">
        <img :src="book.cover" :alt="book.title"
          style="width: 200px; height: 300px; object-fit: cover; border-top-left-radius: 0.5rem; border-top-right-radius: 0.5rem;" />
        <div>
          <h1 class="book-title">{{ book.title }}</h1>
          <p class="author">Author: {{ book.author }}</p>
          <p class="rating">Rating: {{ book.rate }}</p>
        </div>
      </div>
      <div class="description">
        <strong>Description:</strong> {{ book.description }}
      </div>

      <div class="reviews-section border-t pt-6">
        <h1 style="font-size: 1.25rem;      /* Tailwind's text-xl = 20px = 1.25rem */
  font-weight: 600;        /* Tailwind's font-semibold = 600 */
  margin-bottom: 1.25rem;">AI Smart Summary</h1>
        <div class="review-header flex items-center justify-between">
          <div class="user-info flex items-center space-x-3">
            <div
              class="user-avatar w-10 h-10 bg-gray-300 rounded-full flex items-center justify-center text-white font-bold">
            </div>
            <div class="user-name font-medium">Our AI</div>
          </div>
          <div class="review-date text-sm text-gray-500">May 2025</div>
        </div>
        <div class="review-rating mt-2 text-yellow-500">
          <span v-for="n in 5" :key="n" class="star">
            <img v-if="n <= Math.floor(averageRating)" :src="fullStar" alt="Full Star" class="star" />
            <img v-else-if="n === Math.ceil(averageRating) && !Number.isInteger(averageRating)" :src="halfStar"
              alt="Half Star" class="star" />
            <img v-else :src="emptyStar" alt="Empty Star" class="star" style="opacity: 0.3; filter: grayscale(1);" />
          </span>
        </div>
        <div class="review-comment mt-3 text-gray-700 italic">
          “This is a mockup review that summarizes comments from all major book websites. The book is widely appreciated
          for its narrative style and character depth, although some readers noted pacing issues.”
        </div>
      </div>
    </div>

    <div v-else>
      <p class="book-not-found">Book not found.</p>
    </div>

    <div v-if="book.reviews && book.reviews.length" class="rating-summary">
      <h2>Customer Reviews</h2>
      <div class="average-rating">
        <div class="rating-value">{{ averageRating.toFixed(1) }}</div>
        <div class="stars">
          <span v-for="n in 5" :key="n" class="star">
            <img v-if="n <= Math.floor(averageRating)" :src="fullStar" alt="Full Star" class="star" />
            <img v-else-if="n === Math.ceil(averageRating) && !Number.isInteger(averageRatinging)" :src="halfStar"
              alt="Half Star" class="star" />
            <img v-else :src="emptyStar" alt="Empty Star" class="star" style="opacity: 0.3; filter: grayscale(1);" />
          </span>
        </div>
        <div class="review-count">Based on {{ book.reviews.length }} reviews</div>
      </div>

      <div class="ratings-breakdown">
        <div v-for="star in [5, 4, 3, 2, 1]" :key="star" class="rating-bar">
          <span class="rating-label">{{ star }}★</span>
          <div class="rating-bar-background">
            <div class="rating-bar-fill" :style="{ width: (ratingBreakdown[star] / book.reviews.length) * 100 + '%' }">
            </div>
          </div>
          <span class="rating-count">{{ ratingBreakdown[star] }}</span>
        </div>
      </div>
    </div>

    <div class="user-rating-input">
      <h3>Rate this book</h3>
      <StarRating v-model="userRating" :max-stars="5" @ratingData="updateRating" />
      <p class="selected-rating">Your rating: {{ userRating }}</p>
    </div>

    <div v-if="book.reviews && book.reviews.length" class="filter-sort-bar">
      <input v-model="searchTerm" placeholder="Search reviews" class="search-input" />
      <select v-model="sortOrder" class="sort-select">
        <option value="desc">Sort by: Highest rating</option>
        <option value="asc">Sort by: Lowest rating</option>
      </select>
    </div>

    <ul class="filtered-reviews">
      <li v-for="(review, index) in filteredReviews" :key="index" class="filtered-review-card"  @click="$router.push({ name: 'ReviewDetail', query: { review: encodeURIComponent(JSON.stringify(review)) } })"
  style="cursor: pointer;">
        <div class="review-header">
          <div class="user-info">
            <div class="user-avatar"></div>
            <div class="user-name">{{ review.reviewer }}</div>
          </div>
          <div class="review-date">{{ review.date || 'N/A' }}</div>
        </div>
        <div style="display:grid; grid-template-columns:min-content auto; column-gap: 10px;">
          <div class="review-score" style="font-weight: bold;">{{ review.rating }}</div>
          <div class="review-rating">
            <span v-for="n in 5" :key="n" class="star">
              <img v-if="n <= Math.floor(review.rating)" :src="fullStar" alt="Full Star" class="star" />
              <img v-else-if="n === Math.ceil(review.rating) && !Number.isInteger(review.rating)" :src="halfStar"
                alt="Half Star" class="star" />
              <img v-else :src="emptyStar" alt="Empty Star" class="star" style="filter: grayscale(1); opacity: 0.3;" />
            </span>

          </div>
        </div>
        <span class="review-title">{{ review.title || '' }}</span>

        <p class="review-comment">"{{ review.comment }}"</p>
      </li>
    </ul>
  </div>
</template>

<script setup>
import { useRoute } from 'vue-router'
import { ref, computed } from 'vue'
import { bookshelves } from '../data/mockBookshelves.js'
import fullStar from '../assets/FullStar.png'
import halfStar from '../assets/Star.png'
import emptyStar from '@/assets/FullStar.png' 


const route = useRoute()
const book = ref(route.state?.book)

if (!book.value) {
  const bookId = Number(route.params.id)
  const allBooks = Object.values(bookshelves).flat()
  book.value = allBooks.find(b => b.id === bookId)

  if (!book.value) {
    console.warn(`Book with ID ${bookId} not found in fallback.`)
  }
}

const searchTerm = ref('')
const sortOrder = ref('desc')
const userRating = ref(0)

const ratingBreakdown = computed(() => {
  const breakdown = { 5: 0, 4: 0, 3: 0, 2: 0, 1: 0 }
  book.value.reviews.forEach(r => {
    breakdown[r.rating]++
  })
  return breakdown
})

const averageRating = computed(() => {
  const total = book.value.reviews.reduce((sum, r) => sum + r.rating, 0)
  return total / book.value.reviews.length
})

const filteredReviews = computed(() => {
  let results = book.value.reviews.filter(r =>
    r.comment.toLowerCase().includes(searchTerm.value.toLowerCase())
  )
  results.sort((a, b) =>
    sortOrder.value === 'desc' ? b.rating - a.rating : a.rating - b.rating
  )
  return results
})

const updateRating = (rating) => {
  userRating.value = rating
}
</script>

<style scoped>
.book-detail {
  max-width: 960px;
  margin: 0 auto;
  padding: 20px;
}

.book-title {
  font-size: 2.5rem;
  font-weight: bold;
  margin-bottom: 15px;
}

.author,
.rating,
.description {
  font-size: 1.125rem;
  color: #333;
}

.reviews-section {
  margin-top: 40px;
}

.reviews-title {
  font-size: 1.75rem;
  font-weight: bold;
}

.reviews-list {
  list-style: none;
  padding: 0;
}

.review-card {
  background-color: #fff;
  padding: 20px;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  border: 1px solid #e0e0e0;
  margin-bottom: 20px;
}

.review-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 10px;
}

.user-info {
  display: flex;
  align-items: center;
}

.user-avatar {
  background-color: #ccc;
  width: 40px;
  height: 40px;
  border-radius: 50%;
}

.user-name {
  margin-left: 10px;
  font-weight: bold;
}

.review-date {
  color: #777;
  font-size: 0.875rem;
}

.review-rating {
  margin-bottom: 10px;
}

.star {
  color: #f39c12;
  width: 20px;
  height: 20px;
  margin-right: 2px;
}

.review-comment {
  font-style: italic;
  color: #444;
}

.book-not-found {
  color: #e74c3c;
}

.rating-summary {
  margin-top: 40px;
  border-top: 2px solid #f0f0f0;
  padding-top: 20px;
}

.average-rating {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.rating-value {
  font-size: 3rem;
  font-weight: bold;
}

.stars {
  display: flex;
}

.review-count {
  color: #777;
  font-size: 0.875rem;
}

.ratings-breakdown {
  margin-top: 20px;
}

.rating-bar {
  display: flex;
  align-items: center;
  margin-bottom: 10px;
}

.rating-label {
  width: 30px;
  text-align: center;
}

.rating-bar-background {
  flex-grow: 1;
  background-color: #ddd;
  height: 8px;
  border-radius: 5px;
  margin: 0 10px;
}

.rating-bar-fill {
  background-color: #f39c12;
  height: 100%;
  border-radius: 5px;
}

.rating-count {
  width: 40px;
  text-align: right;
}

.filter-sort-bar {
  display: flex;
  justify-content: space-between;
  gap: 20px;
  margin-bottom: 20px;
}

.search-input,
.sort-select {
  padding: 10px;
  border-radius: 6px;
  border: 1px solid #ccc;
  font-size: 1rem;
}

.filtered-reviews {
  list-style: none;
  padding: 0;
}

.filtered-review-card {
  background-color: #fff;
  padding: 15px;
  border-radius: 10px;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  margin-bottom: 15px;
}

.user-rating-input {
  margin: 30px 0;
  padding: 20px;
  background-color: #fdfdfd;
  border: 1px solid #e0e0e0;
  border-radius: 12px;
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.05);
}

.user-rating-input h3 {
  margin-bottom: 10px;
  font-size: 1.25rem;
}

.selected-rating {
  margin-top: 10px;
  font-style: italic;
  color: #555;
}
</style>
