<template>
  <div class="flex">
    <aside class="w-64 pr-6">
      <ul class="space-y-2">
        <li v-for="c in categories" :key="c">
          <button @click="select(c)"
            :class="c===sel?'font-bold text-blue-600':'text-gray-700'">
            {{ c }}
          </button>
        </li>
      </ul>
    </aside>
    <div class="flex-1 grid grid-cols-3 gap-4">
      <BookCard v-for="b in books" :key="b.id" :book="b"/>
    </div>
  </div>
</template>

<script setup>
import BookCard from '@/components/BookCard.vue'
import { ref, onMounted } from 'vue'
import { useBookStore } from '@/store/book'

const store = useBookStore()
const categories=['Romance','Sci-Fi','Mystery'], sel=ref('Romance'), books=ref([])

onMounted(async () => { books.value = await store.fetchByCategory(sel.value) })
async function select(cat) {
  sel.value = cat
  books.value = await store.fetchByCategory(cat)
}
</script>

