<template>
  <div v-if="author">
    <div class="top">
      <AuthorInfo :author="author" />
    </div>

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
import { useRoute } from 'vue-router'
import { authors } from '@/data/authors'
import AuthorInfo from '@/components/Profile/AuthorInfo.vue'
import BookList from '@/components/Profile/BookList.vue'
import Description from '@/components/Profile/Description.vue'


const route = useRoute()
const decodedName = decodeURIComponent(route.params.id)

let author = null
for (const key in authors) {
  if (authors[key].name === decodedName) {
    author = authors[key]
    break
  }
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
</style>
