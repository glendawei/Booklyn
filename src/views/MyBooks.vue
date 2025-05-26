<template>
  <div class="main-container">
    <Sidebar
      :counts="shelfCounts"
      @select="handleShelfSelect"
      @add-shelf="handleAddShelf"
      @remove-shelf="handleRemoveShelf"
    />
    <div class="book-section">
      <h2>{{ selectedShelf }} Books</h2>
      <BookHeader
        :books="filteredBooks"
        @remove-book="removeBook"
      />
      <button @click="showChoose = true" class="add-btn">+ Add Book</button>

      <ChooseBook
        v-if="showChoose"
        :books="booksByShelf['All'] || []"
        @choose="handleChooseBook"
        @cancel="showChoose = false"
      />
    </div>
  </div>
</template>

<script>
import Sidebar from '@/components/MyBooks/Sidebar.vue'
import BookHeader from '@/components/MyBooks/Bookheader.vue'
import ChooseBook from '@/components/MyBooks/ChooseBook.vue'
import { bookshelves } from '@/data/mockBookshelves.js'

export default {
  components: { Sidebar, BookHeader, ChooseBook },
  data() {
    return {
      isLoggedIn: false,
      booksByShelf: {},
      selectedShelf: '',
      nextId: 1000,
      showChoose: false
    }
  },
  computed: {
    shelfCounts() {
      const counts = {}
      for (const [shelf, books] of Object.entries(this.booksByShelf)) {
        counts[shelf] = books.length
      }
      return counts
    },
    filteredBooks() {
      return this.booksByShelf[this.selectedShelf] || []
    }
  },
  created() {
    // 讀取 localStorage 登入狀態
    this.isLoggedIn = localStorage.getItem('loggedIn') === 'true'
    if (this.isLoggedIn) {
      this.booksByShelf = JSON.parse(JSON.stringify(bookshelves))
      this.selectedShelf = Object.keys(this.booksByShelf)[0] || ''
      console.log('初始書櫃：', this.booksByShelf)
    }
  },
  methods: {
    handleShelfSelect(name) {
      this.selectedShelf = name
    },
    handleAddShelf(name) {
      if (!name || this.booksByShelf[name]) {
        alert('書櫃名稱不能為空或重複！')
        return
      }
      this.booksByShelf[name] = []
      this.booksByShelf = { ...this.booksByShelf }
      this.selectedShelf = name
      console.log('新增後書櫃列表：', this.booksByShelf)
    },
    handleRemoveShelf(name) {
      const totalShelves = Object.keys(this.booksByShelf).length
      if (totalShelves <= 1) {
        alert('至少要保留一個書櫃！')
        return
      }

      const bookCount = this.booksByShelf[name].length
      let confirmMessage = `確定要刪除書櫃「${name}」嗎？`
      if (bookCount > 0) {
        confirmMessage = `「${name}」內含 ${bookCount} 本書，確定要刪除？`
      }

      if (confirm(confirmMessage)) {
        delete this.booksByShelf[name]
        this.booksByShelf = { ...this.booksByShelf }
        const remaining = Object.keys(this.booksByShelf)
        this.selectedShelf = remaining[0] || ''
        console.log('刪除後書櫃列表：', this.booksByShelf)
      }
    },
    handleChooseBook(bookList) {
      for (const book of bookList) {
        this.booksByShelf[this.selectedShelf].push({
          ...book,
          id: this.nextId++
        })
      }
      this.showChoose = false
      console.log('新增書籍後列表：', this.booksByShelf)
    },
    removeBook(bookId) {
      const list = this.booksByShelf[this.selectedShelf]
      this.booksByShelf[this.selectedShelf] = list.filter(b => b.id !== bookId)
      console.log('刪除書籍後列表：', this.booksByShelf)
    }
  }
}
</script>
<style scoped>
.main-container {
  display: flex;
  max-width: 1200px;
  margin: 0 auto;
  gap: 20px;
  align-items: flex-start;
  padding: 20px;
}
.book-section {
  flex: 1;
  min-width: 0;
}
.book-section h2 {
  text-align: center;
}
.add-btn {
  display: block;
  margin: 20px auto;
  padding: 8px 16px;
  background-color: #2f6f4e;
  color: white;
  font-weight: bold;
  border: none;
  border-radius: 4px;
}
</style>
