<template>
  <div>
    <h2 class="text-2xl font-bold mb-6">My Bookshelf</h2>
    <table class="w-full table-auto text-left border-collapse">
      <thead>
        <tr class="bg-gray-100">
          <th class="p-2">Cover</th>
          <th class="p-2">Title</th>
          <th class="p-2">Author</th>
          <th class="p-2">Action</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="b in myBooks" :key="b.id" class="border-t">
          <td class="p-2"><img :src="b.cover" class="w-16 h-20 object-cover"/></td>
          <td class="p-2">{{ b.title }}</td>
          <td class="p-2">{{ b.author }}</td>
          <td class="p-2">
            <button @click="remove(b.id)" class="px-3 py-1 bg-red-500 text-white rounded">remove</button>
          </td>
        </tr>
      </tbody>
    </table>
    <button @click="addRandom" class="mt-4 px-4 py-2 bg-green-500 text-white rounded">add</button>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import { useBookStore } from '@/store/book'

const store = useBookStore()
const myBooks = ref([])

onMounted(async () => {
  myBooks.value = await store.fetchMyBooks()
})

function remove(id) {
  store.removeBook(id).then(() =>
    myBooks.value = myBooks.value.filter(b => b.id !== id)
  )
}

function addRandom() {
  store.addRandomBook().then(b =>
    myBooks.value.push(b)
  )
}
</script>

