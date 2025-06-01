<template>
  <div v-if="error" class="not-found">
    {{ error }}
  </div>

  <div v-else-if="author">
    <div class="top">
      <AuthorInfo :author="author" />
    </div>

    <button @click="followAuthor" class="follow-button">
      {{ isFollowing ? 'Following' : 'Follow' }}
    </button>

    <div class="profile-page">
      <div class="left">
        <Description :author="author" />
      </div>
      <div class="right">
        <BookList :books="books" />
      </div>
    </div>
  </div>

  <div v-else>
    Loading...
  </div>
</template>


<script setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import AuthorInfo from '@/components/Profile/AuthorInfo.vue'
import Description from '@/components/Profile/Description.vue'
import BookList from '@/components/Profile/BookList.vue'

// 路由
const route = useRoute()
const authorId = route.params.id

// 狀態管理
const author = ref(null)
const books = ref([])
const isFollowing = ref(false)
const error = ref(null)

// 取得作者資料
async function fetchAuthor() {
  try {
    const res = await fetch(`http://localhost:8080/authors/${authorId}`)
    if (!res.ok) {
      error.value = res.status === 404 ? 'Author not found.' : 'Internal server error.'
      return
    }

    const data = await res.json()
    author.value = data

    // 拿到作者名稱後，取得他的作品
    await fetchBooksByAuthor(data.name)
  } catch (e) {
    console.error(e)
    error.value = 'Network error.'
  }
}

// 取得該作者作品
async function fetchBooksByAuthor(authorName) {
  try {
    const query = encodeURIComponent(authorName)
    const url = `http://localhost:8080/books?author_name=${query}`
    console.log('📘 API URL:', url) // 🧪 Step 1：印出實際 API 呼叫路徑

    const res = await fetch(url)
    const rawText = await res.text()
    console.log('📘 Raw response:', rawText) // 🧪 Step 2：印出原始回應內容

    if (!res.ok) {
      console.warn('⚠️ books response not OK')
      books.value = []
      return
    }

    const data = JSON.parse(rawText)
    console.log('📘 Parsed books:', data) // 🧪 Step 3：印出 JSON 解析結果
    books.value = data
  } catch (e) {
    console.error('❌ Error fetching books:', e)
    books.value = []
  }
}


// 追蹤作者
function followAuthor() {
  isFollowing.value = !isFollowing.value
  alert(`${isFollowing.value ? 'You followed' : 'You unfollowed'} ${author.value?.name}`)
}

// 初始化
onMounted(() => {
  fetchAuthor()
})
</script>

<style scoped>
.profile-page {
  display: flex;
  flex-direction: row;
  background: #f9fafb;
  min-height: 100vh;
  padding: 1.5rem;
  gap: 1.5rem;
}

.top {
  margin-bottom: 1.5rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.left {
  width: 33.3333%;
}

.right {
  width: 66.6666%;
}

.not-found {
  padding: 5rem;
  text-align: center;
  font-size: 1.25rem;
  color: #e11d48;
}


.follow-button {
  align-self: flex-end;       /* 如果父層是 flex-column 可用這行 */
  margin-left: auto;          /* 💡 靠右關鍵：左邊推到底 */
  display: block;             /* 確保它不繼承 inline 行為 */
  padding: 0.5rem 1rem;
  font-weight: bold;
  border: none;
  border-radius: 0.5rem;
  background-color: #ddf089;
  color: rgb(198, 118, 6);
  cursor: pointer;
  transition: background-color 0.2s;
}

.follow-button:hover {
  background-color: #cbb31b;
}

</style>
