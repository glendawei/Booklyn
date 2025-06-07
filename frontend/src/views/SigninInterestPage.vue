<template>
  <div class="interest-page">
    <h2>Welcome to booklyn! Let us get to know you first.</h2>
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
    <button type="button" class="submit-btn" @click="submit">Get started</button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'


const router = useRouter()
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
  console.log('[debug] rawUser =', rawUser)

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
    alert('興趣已更新！')
    router.push('/profile-settings/personal-info')
  } catch (err) {
    console.error('❌ 偏好更新失敗:', err)
    alert('更新失敗，請稍後再試')
  }
}

</script>


  
<style scoped>
  .interest-page {
    max-width: 800px;
    margin: 60px auto;
    text-align: center;
    padding: 20px;
  }
  
  h2 {
    margin-bottom: 30px;
    font-size: 28px;
    font-weight: bold;
    color: #333;
  }
  
  .interest-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 15px;
    margin-bottom: 40px;
  }
  
  .interest-item {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    padding: 20px;
    border: 2px solid #e0e0e0;
    border-radius: 15px;
    cursor: pointer;
    transition: all 0.3s;
    background-color: #fafafa;
  }
  
  .interest-item:hover {
    border-color: #606C38;
  }
  
  .interest-item.selected {
    background-color: #606C38;
    color: white;
    border-color: #606C38;
  }
  
  .icon {
    font-size: 24px;
    margin-bottom: 8px;
  }
  
  .label {
    font-size: 16px;
    font-weight: 500;
  }
  
  .submit-btn {
    background-color: #DDA15E;
    color: white;
    padding: 14px 40px;
    border: none;
    border-radius: 8px;
    font-size: 16px;
    cursor: pointer;
    transition: background-color 0.3s;
  }
  
  .submit-btn:hover {
    background-color: #BC6C25;
  }
  </style>
  