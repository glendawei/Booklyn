<!-- BookHeader.vue -->
<template>
  <div class="header-wrapper">
    <table class="header-table">
      <thead>
        <tr>
          <th>Title</th>
          <th>Cover</th>
          <th>Author</th>
          <th>Rate</th>
        </tr>
      </thead>
      <tbody>
        <BookCard
          v-for="book in books"
          :key="book.id"
          :book="book"
          @remove-book="$emit('remove-book', $event)"
        />
      </tbody>
    </table>
  </div>
</template>

<script>
import BookCard from './BookCard.vue'

export default {
  name: 'BookHeader',
  components: { BookCard },
  props: {
    books: {
      type: Array,
      required: true
    }
  }
}
</script>


<style scoped>
.header-wrapper {
  width: 100%;
  overflow-x: auto;
}

.header-table {
  width: 100%;
  border-collapse: collapse;
  margin-bottom: 10px;
}

/* ✅ 表頭背景深綠 + 白字 */
.header-table thead {
  background-color: #113318;
}
.header-table th {
  width: 25%;
  color: white;
  text-align: left;
  padding: 8px;
}

/* ✅ 表格內容白底 + 深色字 */
.header-table tbody {
  background-color: white;
}
.header-table td {
  width: 25%;
  color: #1f2937; /* 深灰色文字 */
  padding: 8px;
}

</style>
