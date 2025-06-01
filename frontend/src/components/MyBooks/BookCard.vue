
<template>
  <tr>
    <td>
      <router-link
        :to="`/books/${book.bookId}`"
        class="text-link"
        @click="storeBookId(book.bookId)"
      >
        {{ book.title }}
      </router-link>
    </td>
    <td>
      <router-link
        :to="`/books/${book.bookId}`"
        @click="storeBookId(book.bookId)"
      >
        <img :src="book.cover" alt="Book Cover" style="width: 60%;" />
      </router-link>
    </td>
    <td>
      <router-link
        :to="`/authors/${book.author_id}`"
        class="text-link"
      >
        <strong>{{ book.author }}</strong>
      </router-link>
    </td>
    <td style="position: relative;">
      {{ book.rate }}
      <button @click="$emit('remove-book', book.id)" class="close-btn">X</button>
    </td>
  </tr>
</template>

<script>
export default {
  name: 'BookCard',
  props: {
    book: {
      type: Object,
      required: true
    }
  },
  methods: {
    storeBookId(id) {
      localStorage.setItem('selectedBookId', id)
    }
  }
}
</script>





<style scoped>
.card-cell {
  padding: 0;
}

.card-wrapper {
  position: relative;
  padding: 16px;
  border: 1px solid #ccc;
  border-radius: 8px;
}

.close-btn {
  position: absolute;
  top: 0px;
  right: 0px;
  background: none;
  border: none;
  color: rgb(23, 55, 43);
  font-size: 18px;
  cursor: pointer;
}

.text-link {
  color: #062118;
}
</style>
