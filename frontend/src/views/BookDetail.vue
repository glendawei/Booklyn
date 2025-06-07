<template>
  <div>
    <!-- Floating Review Detail Side Panel -->
    <div v-if="selectedReview" class="drawer-overlay">
      <div class="drawer-panel">
        <button class="drawer-close" @click="selectedReview = null">×</button>

        <h2 class="drawer-title">{{ selectedReview?.title || 'N/A' }}</h2>
        <p class="drawer-meta">
          Reviewed by {{ selectedReview?.reviewer || "N/A" }} on
          {{ selectedReview?.date || "N/A" }} from {{ selectedReview?.source || "N/A" }}
        </p>
        <div style="display: grid; grid-template-columns: auto 1fr; column-gap: 10px;">
          <p class="drawer-rating">{{ selectedReview?.rating ?? 'N/A' }}</p>
          <div class="review-rating">
            <span v-for="n in 5" :key="n" class="star">
              <img v-if="n <= Math.floor(selectedReview?.rating || 0)" :src="fullStar" alt="Full Star" class="star" />
              <img v-else-if="n === Math.ceil(selectedReview?.rating || 0) && !Number.isInteger(selectedReview?.rating)"
                :src="halfStar" alt="Half Star" class="star" />
              <img v-else :src="emptyStar" alt="Empty Star" class="star" style="filter: grayscale(1); opacity: 0.3" />
            </span>
          </div>
        </div>
        <p class="drawer-comment">{{ selectedReview?.comment || 'No comment available.' }}</p>
        <div style="display: grid; grid-template-columns: 150px auto;">
          <ProgressCircle height="150" width="150" color="#BC6C25" :name="ad" :progress="selectedReview?.credibility_score || 0"
            style="padding:20px;" />
          <div style="padding-top:65px; color: #BC6C25;">Is this AI-generated?</div>
        </div>
      </div>
    </div>

    <div class="book-detail">
      <div v-if="book">
        <div style="display: grid; grid-template-columns: 200px 1fr; column-gap: 50px; margin-bottom: 25px;">
          <img :src="book.cover_url || '/default-cover.png'" :alt="book.title"
            style="width: 200px; height: 300px; object-fit: cover; border-top-left-radius: 0.5rem; border-top-right-radius: 0.5rem;" />

          <div>
            <div style="display: grid; grid-template-columns: auto 30px;">
              <h1 class="book-title">{{ book.title }}</h1>
              <button v-if="isLoggedIn" @click="addToReadingList"
                style="height: 30px; border-radius: 5px; border: none; background-color: #BC6C25; color: white; font-size: 20px;">
                +
              </button>


            </div>
            <p class="author">
              Author:
              <template v-if="book?.authors?.length">
                <span v-for="(author, index) in book.authors" :key="author.author_id">
                  <router-link :to="`/authors/${author.author_id}`" class="author-link">
                    {{ author.name }}
                  </router-link>
                  <span v-if="index < book.authors.length - 1">, </span>
                </span>
              </template>
              <span v-else>N/A</span>
            </p>


            <p class="rating">Rating: {{ book.rate ?? 'N/A' }}</p>
          </div>
        </div>

        <div class="description">
          <strong>Description:</strong> {{ book.description || 'No description available.' }}
        </div>

        <div class="reviews-section border-t pt-6">
          <h1 style="font-size: 1.25rem; font-weight: 600; margin-bottom: 1.25rem;">
            AI Smart Summary
          </h1>
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
              <img v-if="n <= Math.floor(averageRating || 0)" :src="fullStar" alt="Full Star" class="star" />
              <img v-else-if="n === Math.ceil(averageRating || 0) && !Number.isInteger(averageRating)" :src="halfStar"
                alt="Half Star" class="star" />
              <img v-else :src="emptyStar" alt="Empty Star" class="star" style="opacity: 0.3; filter: grayscale(1)" />
            </span>
          </div>
          <div class="review-comment mt-3 text-gray-700 italic">
            {{ book.ai_summary }}
          </div>
        </div>
      </div>

      <div v-else>
        <p class="book-not-found">Book not found.</p>
      </div>

      <div v-if="book && book.reviews && book.reviews.length" class="rating-summary">
        <h2>Customer Reviews</h2>
        <div class="average-rating">
          <div class="rating-value">{{ averageRating?.toFixed(1) || '0.0' }}</div>
          <div class="stars">
            <span v-for="n in 5" :key="n" class="star">
              <img v-if="n <= Math.floor(averageRating || 0)" :src="fullStar" alt="Full Star" class="star" />
              <img v-else-if="n === Math.ceil(averageRating || 0) && !Number.isInteger(averageRating)" :src="halfStar"
                alt="Half Star" class="star" />
              <img v-else :src="emptyStar" alt="Empty Star" class="star" style="opacity: 0.3; filter: grayscale(1)" />
            </span>
          </div>
          <div class="review-count">
            Based on {{ book.reviews.length }} reviews
          </div>
        </div>

        <div class="ratings-breakdown">
          <div v-for="star in [5, 4, 3, 2, 1]" :key="star" class="rating-bar">
            <span class="rating-label">{{ star }}★</span>
            <div class="rating-bar-background">
              <div class="rating-bar-fill" :style="{
                width: ((ratingBreakdown[star] || 0) / book.reviews.length) * 100 + '%',
              }"></div>
            </div>
            <span class="rating-count">{{ ratingBreakdown[star] || 0 }}</span>
          </div>
        </div>
      </div>

      <div class="user-rating-input">
        <h3>Rate this book</h3>
        <StarRating v-model="userRating" :max-stars="5" @ratingData="updateRating" />
        <p class="selected-rating">Your rating: {{ userRating }}</p>
      </div>

      <div v-if="book && book.reviews && book.reviews.length" class="filter-sort-bar">
        <input v-model="searchTerm" placeholder="Search reviews" class="search-input" />
        <select v-model="sortOrder" class="sort-select">
          <option value="desc">Sort by: Highest rating</option>
          <option value="asc">Sort by: Lowest rating</option>
        </select>
      </div>

      <ul v-if="book && filteredReviews.length" class="filtered-reviews">
        <li v-for="(review, index) in filteredReviews" :key="index" class="filtered-review-card">
          <div class="review-header">
            <div class="user-info">
              <div class="user-avatar"></div>
              <div class="user-name">{{ review.reviewer || 'Anonymous' }}</div>
            </div>
            <div class="review-date">{{ review.date || "N/A" }}</div>
          </div>
          <div style="display: grid; grid-template-columns: min-content auto; column-gap: 10px;">
            <div class="review-score" style="font-weight: bold">
              {{ review.rating ?? 0 }}
            </div>
            <div class="review-rating">
              <span v-for="n in 5" :key="n" class="star">
                <img v-if="n <= Math.floor(review.rating || 0)" :src="fullStar" alt="Full Star" class="star" />
                <img v-else-if="n === Math.ceil(review.rating || 0) && !Number.isInteger(review.rating)" :src="halfStar"
                  alt="Half Star" class="star" />
                <img v-else :src="emptyStar" alt="Empty Star" class="star" style="filter: grayscale(1); opacity: 0.3" />
              </span>
            </div>
          </div>
          <span class="review-title">{{ review.title || "" }}</span>
          <p class="review-comment">"{{ review.comment || '' }}"</p>
          <div style="display: grid; grid-template-columns:1fr 1fr 1fr;">
            <button class="styled-button" @click="showDetailPanel(review)">
              View Details
            </button>
            <div class="review-upvote">
              <button class="styled-button upvote-button" :disabled="reviewUpvotes[review.title]"
                @click="upvoteReview(review)">
                Upvote
              </button>
              <span class="upvote-count">{{ review.upvotes || 0 }} upvotes</span>
            </div>
          </div>

          <!-- Comment List -->
          <div class="comments">
            <p v-for="comment in comments[review.review_id]" :key="comment.comment_id">
              <strong>user_{{ comment.user_id || 'Anonymous' }}:</strong> {{ comment.content }}
            </p>
          </div>



          <!-- New Comment Input -->
          <div class="mt-3 flex items-center gap-2">
            <input type="text" v-model="newComments[review.review_id]"
              class="flex-grow p-2 border rounded shadow-sm focus:outline-none focus:ring"
              placeholder="Write a comment..." />
            <button class="bg-green-500 text-white px-4 py-2 rounded" @click="submitComment(review)">
              Submit
            </button>
          </div>
        </li>
      </ul>
    </div>
  </div>
</template>

<script setup>
import axios from 'axios'

import { useRoute } from "vue-router";
import { ref, computed, onMounted, reactive } from "vue";
import fullStar from "../assets/FullStar.png";
import halfStar from "../assets/Star.png";
import emptyStar from "@/assets/FullStar.png";
import ProgressCircle from "../components/ProgressCircle.vue";

const route = useRoute();
const book = ref(null);
const selectedReview = ref(null);
const isDetailPanelVisible = ref(false);
const searchTerm = ref("");
const sortOrder = ref("desc");
const userRating = ref(0);

const bookId = Number(route.params.id);
// Track new comment input per review
const newComments = reactive({});

// 🔹 呼叫 API 取得書籍資訊
const fetchBook = async () => {
  try {
    const response = await fetch(`http://localhost:8080/books/${bookId}`);
    if (!response.ok) throw new Error("Book not found");
    const data = await response.json();

    // 🔁 將 review 的 content 映射到 comment，credibility_score 映射到 aiRating，profile_name 映射 reviewer，review_time 映射 date
    data.reviews = data.reviews?.map(r => ({
      ...r,
      comment: r.content,
      aiRating: parseFloat(r.credibility_score) || 0,
      reviewer: r.profile_name || "Anonymous",
      date: r.review_time || "",
      source: r.source || ""
    })) ?? [];

    if (data?.reviews) {
      data.reviews.forEach((review) => {
        newComments[review.review_id] = '';
      });
    }

    book.value = data;

    console.log("📖 書名:", book.value.title);
    console.log("🖊️ 作者:", book.value.authors?.map((a) => a.name).join(", "));
    console.log("📚 分類:", book.value.categories?.join(", "));
    console.log("📝 描述:", book.value.description);
    console.log("📷 封面 URL:", book.value.cover_url);
    console.log("💬 評論數:", book.value.reviews?.length);
    console.log("📝 第一則評論:", book.value.reviews?.[0]?.comment);
    console.log("🤖 AI 整理:", book.value.ai_summary);
    console.log("👤 第一則評論者:", book.value.reviews?.[0]?.reviewer);
    console.log("🌐 第一則評論來源:", book.value.reviews?.[0]?.source);
  } catch (err) {
    console.error("❌ 無法取得書籍資料：", err.message);
  }
};

const isLoggedIn = localStorage.getItem('loggedIn') === 'true';

const ratingBreakdown = computed(() => {
  if (!book.value || !book.value.reviews) return {};
  const breakdown = { 5: 0, 4: 0, 3: 0, 2: 0, 1: 0 };
  book.value.reviews.forEach((r) => {
    const floored = Math.floor(r.rating);
    breakdown[floored]++;
  });
  return breakdown;
});

const addToReadingList = async () => {
  const userId = localStorage.getItem('user_id');
  console.log('📌 目前儲存在 localStorage 的 user_id 是：', userId);

  try {
    const payload = {
      book_id: book.value.book_id,
      note: "想讀這本書",
      status: "want-to-read",
    };
    const response = await axios.post(`http://localhost:8080/users/${userId}/reading-list`, payload);
    console.log("✅ 已加入閱讀清單", response.data);
    alert("已成功加入閱讀清單！");
  } catch (err) {
    console.error("❌ 加入閱讀清單失敗：", err.message);
    alert("加入失敗，請稍後再試。");
  }
};

const averageRating = computed(() => {
  if (!book.value || !book.value.reviews || book.value.reviews.length === 0) return 0;
  const total = book.value.reviews.reduce((sum, r) => sum + r.rating, 0);
  return total / book.value.reviews.length;
});

const filteredReviews = computed(() => {
  if (!book.value || !book.value.reviews) return [];
  let results = book.value.reviews.filter((r) =>
    r.comment?.toLowerCase().includes(searchTerm.value.toLowerCase())
  );
  results.sort((a, b) =>
    sortOrder.value === "desc" ? b.rating - a.rating : a.rating - b.rating
  );
  return results;
});

const updateRating = (rating) => {
  userRating.value = rating;
};

const showDetailPanel = (review) => {
  selectedReview.value = review;
  isDetailPanelVisible.value = true;
};

const reviewUpvotes = ref({});

const upvoteReview = async (review) => {
  const userId = parseInt(localStorage.getItem('user_id'));
  if (!reviewUpvotes.value[review.review_id]) {
    try {
      const payload = {
        helpful: true,
        user_id: userId
      };
      const response = await axios.post(`http://localhost:8080/reviews/${review.review_id}/vote`, payload);
      console.log("✅ 已按讚", response.data);
      // Optimistically update UI
      review.upvotes = (review.upvotes || 0) + 1;
      reviewUpvotes.value[review.review_id] = true;
    } catch (error) {
      console.error("Upvote failed:", error);
    }
  }
};

// Post a new comment
const submitComment = async (review) => {
  const userId = parseInt(localStorage.getItem('user_id'))
  const content = newComments[review.review_id]

  if (!content) return

  try {
    const res = await axios.post(`http://localhost:8080/reviews/${review.review_id}/comments`, {
      content: content,
      parent_id: null,
      user_id: userId,
    })
    console.log("✅ 已評論", res.data);
    // Add new comment optimistically
    if (!review.comments) review.comments = []
    review.comments.push(res.data)
    newComments[review.review_id] = ''
  } catch (err) {
    console.error('Failed to post comment:', err)
  }
}

const comments = reactive({})

const getCommentsForReview = async (review) => {
  if (!(review.review_id in comments)) {
    comments[review.review_id] = null // to avoid duplicate fetches
    const temp = parseInt(review.review_id)

    try {
      const res = await axios.get(`http://localhost:8080/reviews/${temp}/comments`)
      comments[review.review_id] = res.data
      console.log("comments logged", comments[review.review_id])
    } catch (err) {
      if (err.response && err.response.status === 404) {
        comments[review.review_id] = [] // No comments for this review
        console.log(`No comments found for review ${temp}. Set to empty array.`)
      } else {
        console.error(`Failed to fetch comments for review ${temp}:`, err)
        throw err // Optional: rethrow for non-404 errors
      }
    }
  }
  return comments[review.review_id]
}

onMounted(async () => {
  await fetchBook()
  // book.value.reviews may still be undefined/null, so guard it
  if (book.value?.reviews) {
    for (const review of book.value.reviews) {
      await getCommentsForReview(review)
    }
  }
})


</script>



<style scoped>
/* [Styling is unchanged — kept from your original version] */
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

.slide-enter-from,
.slide-leave-to {
  transform: translateX(100%);
  opacity: 0;
}

.slide-enter-to,
.slide-leave-from {
  transform: translateX(0);
  opacity: 1;
}

.slide-enter-active,
.slide-leave-active {
  transition: all 0.3s ease;
}

.styled-button {
  margin-top: 1rem;
  padding: 0.5rem 1.2rem;
  font-size: 0.875rem;
  font-weight: 500;
  color: white;
  background-color: #bc6c25;
  /* Tailwind's blue-600 */
  border: none;
  border-radius: 0.5rem;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.1);
  transition:
    background-color 0.2s ease,
    transform 0.1s ease;
  cursor: pointer;
}

.styled-button:hover {
  background-color: #92531d;
  /* Tailwind's blue-700 */
}

.styled-button:focus {
  outline: 2px solid #dda15e;
  /* Tailwind's blue-400 */
}

.drawer-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.3);
  /* dimmed background */
  z-index: 9999;
}

.drawer-panel {
  position: fixed;
  top: 0;
  right: 0;
  height: 100vh;
  width: 400px;
  max-width: 100%;
  background-color: #fff;
  box-shadow: -4px 0 20px rgba(0, 0, 0, 0.2);
  padding: 24px;
  overflow-y: auto;
  animation: slideIn 0.3s ease-out;
  z-index: 10000;
}

.drawer-close {
  position: absolute;
  top: 12px;
  right: 16px;
  font-size: 1.5em;
  background: none;
  border: none;
  cursor: pointer;
  color: #666;
}

.drawer-close:hover {
  color: #000;
}

.drawer-title {
  font-size: 1.5em;
  margin-bottom: 10px;
}

.drawer-meta {
  font-size: 0.9em;
  color: #555;
  margin-bottom: 16px;
}

.drawer-rating {
  font-weight: bold;
  margin-bottom: 12px;
}

.drawer-comment {
  font-size: 1em;
  color: #333;
}

@keyframes slideIn {
  from {
    transform: translateX(100%);
  }

  to {
    transform: translateX(0%);
  }
}

.upvote-button {
  background-color: #f4a261;
  color: white;
  border: none;
  padding: 0.5rem 1.2rem;
  margin-top: 1rem;
  margin-left: 0.5rem;
  border-radius: 0.5rem;
  cursor: pointer;
}

.upvote-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.upvote-count {
  margin-left: 10px;
  font-size: 0.9rem;
  color: #444;
}
</style>