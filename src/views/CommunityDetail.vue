<template>
  <div>
    <img :src="thread.image" class="w-full h-64 object-cover mb-4"/>
    <h2 class="text-2xl font-bold mb-2">{{ thread.title }}</h2>
    <div class="space-y-4">
      <CommentCard v-for="c in comments" :key="c.id" :comment="c"/>
    </div>
    <CommentForm @posted="loadComments"/>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import CommentCard from '@/components/CommentCard.vue'
import CommentForm from '@/components/CommentForm.vue'
import { useThreadStore } from '@/store/thread'

const route = useRoute()
const store = useThreadStore()
const thread=ref({}), comments=ref([])

async function loadComments() {
  const data = await store.fetchComments(route.params.id)
  comments.value = data
}

onMounted(async () => {
  thread.value = await store.fetchThread(route.params.id)
  loadComments()
})
</script>

