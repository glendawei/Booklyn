<template>
  <div class="flex space-x-8">
    <ProfileCard :user="user" />
    <div class="flex-1 space-y-8">
      <section>
        <h3 class="font-semibold mb-2">My Books</h3>
        <BookCard v-for="b in userBooks" :key="b.id" :book="b" />
      </section>
      <section>
        <h3 class="font-semibold mb-2">My Community</h3>
        <ThreadCard v-for="t in userThreads" :key="t.id" :thread="t" />
      </section>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import ProfileCard from '@/components/ProfileCard.vue'
import BookCard from '@/components/BookCard.vue'
import ThreadCard from '@/components/ThreadCard.vue'
import { useUserStore } from '@/store/user'

const store = useUserStore()
const user = ref({}), userBooks=ref([]), userThreads=ref([])

onMounted(async () => {
  user.value = await store.fetchUser()
  userBooks.value = await store.fetchUserBooks()
  userThreads.value = await store.fetchUserThreads()
})
</script>
