<template>
  <div class="interest-page">
    <h2>Revise your book preference</h2>
    <div class="interest-grid">
      <div
        v-for="interest in interests"
        :key="interest.name"
        :class="['interest-item', selected.includes(interest.name) ? 'selected' : '']"
        @click="toggleInterest(interest.name)"
      >
        <span class="icon">{{ interest.icon }}</span>
        <span class="label">{{ interest.name }}</span>
      </div>
    </div>
    <button type="button" class="submit-btn" @click="submit">Save Preferences</button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import axios from 'axios'

const interests = ref([])
const selected = ref([])

const emojiMap = [
  '📖', '✝️', '🏛️', '🧬', '🏀', '🧘‍♀️', '📚',
  '💰', '🧒', '🎨', '🌍', '📐', '🔬'
]

onMounted(async () => {
  console.log('📌 onMounted trigger')
  const rawUser = localStorage.getItem('user')
  console.log('🧩 user from localStorage:', rawUser)

  try {
    const response = await axios.get('http://localhost:8080/books')
    const books = response.data

    const categorySet = new Set()
    books.forEach(book => {
      (book.categories || []).forEach(cat => categorySet.add(cat))
    })

    interests.value = Array.from(categorySet).map((name, index) => ({
      name,
      icon: emojiMap[index % emojiMap.length]
    }))
  } catch (err) {
    console.error('📚 無法取得書籍分類', err)
  }

  const user = JSON.parse(rawUser || '{}')
  if (user && user.preferred_topics) {
    selected.value = [...user.preferred_topics]
  }
})

function toggleInterest(name) {
  const idx = selected.value.indexOf(name)
  idx === -1
    ? selected.value.push(name)
    : selected.value.splice(idx, 1)
}

async function submit() {
  console.log('✅ submit called')
  const rawUser = localStorage.getItem('user')
  if (!rawUser) {
    alert('請先登入！')
    return
  }

  const user = JSON.parse(rawUser)
  if (selected.value.length < 1 || selected.value.length > 5) {
    alert('請選擇 1–5 個興趣')
    return
  }

  try {
    const response = await axios.patch(`http://localhost:8080/users/${user.user_id}`, {
      preferred_topics: selected.value
    })

    localStorage.setItem('user', JSON.stringify(response.data))
    alert('偏好已更新！')
  } catch (err) {
    console.error('❌ 偏好更新失敗:', err)
    alert('更新失敗，請稍後再試')
  }
}
</script>

<style scoped>
.interest-page {
  padding: 40px;
  color: #99a56e;
  background-color: #99a56e;
  min-height: 100vh;
  text-align: center;
}

h2 {
  font-size: 28px;
  margin-bottom: 10px;
  color: #FEFAE0;
}

.interest-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  gap: 20px;
  margin-bottom: 30px;
}

.interest-item {
  background-color: #FEFAE0;
  border: 2px solid transparent;
  border-radius: 12px;
  padding: 20px;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 2px 2px 5px rgba(0,0,0,0.1);
}

.interest-item:hover {
  border-color: #DDA15E;
}

.interest-item.selected {
  background-color: #283618;
  color: white;
  border-color: #BC6C25;
}

.icon {
  font-size: 24px;
  display: block;
  margin-bottom: 10px;
}

.label {
  font-size: 16px;
  font-weight: 500;
}

.submit-btn {
  background-color: #BC6C25;
  color: white;
  padding: 12px 28px;
  font-size: 16px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.3s;
}

.submit-btn:hover {
  background-color: #A95A1D;
}
</style>
