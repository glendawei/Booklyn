<template>
  <div v-if="author">
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
        <BookList :books="author.books" />
      </div>
    </div>
  </div>

  <div v-else class="not-found">
    Author not found.
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRoute } from 'vue-router'
import { authors } from '@/data/authors'
import AuthorInfo from '@/components/Profile/AuthorInfo.vue'
import BookList from '@/components/Profile/BookList.vue'
import Description from '@/components/Profile/Description.vue'

// 取得路由參數中的 id，並解碼成作者名稱
const route = useRoute()
const decodedName = decodeURIComponent(route.params.id)

// 根據名稱找對應的作者
let author = null
for (const key in authors) {
  if (authors[key].name === decodedName) {
    author = authors[key]
    break
  }
}

// 追蹤邏輯
const isFollowing = ref(false)
function followAuthor() {
  isFollowing.value = !isFollowing.value
  console.log(`${isFollowing.value ? 'Followed' : 'Unfollowed'} ${decodedName}`)
  alert(`${isFollowing.value ? 'You followed' : 'You unfollowed'} ${decodedName}`)
}
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
