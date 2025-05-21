<template>
  <div class="review-detail">
    <div class="filtered-review-card" v-if="review">
      <button @click="goBack" class="back-btn">← Back</button>

    
      <div class="review-header flex items-center justify-between">
        <div class="user-info flex items-center space-x-3">
          <div class="user-avatar w-10 h-10 bg-gray-300 rounded-full flex items-center justify-center text-white font-bold"></div>
          <div class="user-name font-medium">{{ review.reviewer || 'Anonymous' }}</div>
        </div>
        <div class="review-date text-sm text-gray-500">{{ review.date || 'N/A' }}</div>
      </div>

 
      <div class="review-rating mt-2 text-yellow-500">
        <span v-for="n in 5" :key="n" class="star">
          <img v-if="n <= Math.floor(review.rating)" :src="fullStar" alt="Full Star" class="star" />
          <img v-else-if="n === Math.ceil(review.rating) && !Number.isInteger(review.rating)" :src="halfStar" alt="Half Star" class="star" />
          <img v-else :src="emptyStar" alt="Empty Star" class="star" style="opacity: 0.3; filter: grayscale(1);" />
        </span>
      </div>

    
      <span class="review-title" v-if="review.title">{{ review.title }}</span>

     
      <div class="review-comment mt-3 text-gray-700 italic">
        “{{ review.comment }}”
      </div>

      
      <div class="review-meta mt-6">
        <div><strong>Reputation:</strong> {{ review.reputation || 'N/A' }}</div>
        <div><strong>Source:</strong> {{ review.source || 'Unknown' }}</div>
        <div><strong>Publish Date:</strong> {{ review.date || 'Unknown' }}</div>
      </div>

      
      <div class="review-confidence mt-6" v-if="review.confidence">
        <h3 style="margin-bottom: 12px;">Confidence Breakdown</h3>
        <ul class="confidence-list">
            <li v-for="(value, label) in review.confidence" :key="label" class="confidence-item">
            <div class="label-with-icon">
                <span class="label-text">{{ label }}</span>
                <span class="info-icon">i</span>
            </div>
            <div class="confidence-right">
                <span class="color-circle" :style="{ backgroundColor: confidenceColors[label] }"></span>
                <span>{{ value }}%</span>
            </div>
            </li>
        </ul>
        </div>
    </div>

    <div v-else>
      <p class="text-red-500 font-semibold">Review not found or invalid.</p>
    </div>
  </div>
</template>

<script setup>
import { useRoute, useRouter } from 'vue-router'
import { ref , onMounted} from 'vue'
import fullStar from '@/assets/FullStar.png'
import halfStar from '@/assets/Star.png'
import emptyStar from '@/assets/FullStar.png'

const router = useRouter()
const route = useRoute()
const review = ref(null)

onMounted(() => {
  const raw = route.query.review
  if (raw) {
    try {
        // 這邊等後面ai判讀進行串接
      review.value = JSON.parse(decodeURIComponent(raw))
        // 這個是fake data 為了要確認有沒有成功
      if (!review.value.confidence) {
        review.value.confidence = {
          "AI-generated": 0,
          "AI-generated & AI-refined": 5,
          "Human-written & AI-refined": 70,
          "Human-written": 25
        }
      }

    } catch (err) {
      console.warn('Failed to parse review from query:', err)
    }
  }
})

const goBack = () => {
  router.back()
}

const confidenceColors = {
  "AI-generated": "#f97316",
  "AI-generated & AI-refined": "#fde047",
  "Human-written & AI-refined": "#99f6e4",
  "Human-written": "#c4b5fd"
}
</script>


<style scoped>
.review-detail {
  max-width: 960px;
  margin: 0 auto;
  padding: 30px;
  font-family: sans-serif;
}

.filtered-review-card {
  background-color: #fff;
  padding: 20px;
  border-radius: 12px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  border: 1px solid #e0e0e0;
  margin-bottom: 20px;
}

.back-btn {
  display: inline-block;
  margin-bottom: 20px;
  color: #3b82f6;
  text-decoration: none;
  font-size: 1rem;
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
  width: 20px;
  height: 20px;
  margin-right: 2px;
}

.review-title {
  display: block;
  font-weight: bold;
  margin-top: 6px;
  font-size: 1.1rem;
}

.review-comment {
  font-style: italic;
  color: #444;
  margin-top: 8px;
}

.review-meta {
  margin-top: 20px;
  color: #555;
  line-height: 1.6;
}

.review-confidence {
  margin-top: 30px;
}

.confidence-list {
  list-style: none;
  padding-left: 0;
}

.color-dot {
  display: inline-block;
  width: 12px;
  height: 12px;
  border-radius: 50%;
  margin-right: 8px;
}

.confidence-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
  font-size: 1rem;
}

.label-with-icon {
  display: flex;
  align-items: center;
  gap: 6px;
}

.label-text {
  font-weight: 500;
}

.info-icon {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  font-size: 0.75rem;
  background: black;
  color: white;
  width: 16px;
  height: 16px;
  border-radius: 50%;
  font-family: sans-serif;
}

.confidence-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.color-circle {
  width: 14px;
  height: 14px;
  border-radius: 50%;
}
</style>
