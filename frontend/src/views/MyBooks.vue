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
import axios from 'axios'

const baseURL = 'http://localhost:8080'

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
  async created() {
  this.isLoggedIn = localStorage.getItem('loggedIn') === 'true'
  if (!this.isLoggedIn) return

  const userId = localStorage.getItem('user_id')
  if (!userId) {
    console.warn('⚠️ 找不到 user_id')
    return
  }

  try {
    const res = await axios.get(`http://localhost:8080/users/${userId}/reading-list`)
    const rawList = res.data

    const detailedList = await Promise.all(
      rawList.map(async (item) => {
        try {
          const bookRes = await axios.get(`http://localhost:8080/books/${item.book_id}`)
          const book = bookRes.data
          return {
            id: item.item_id,
            title: book.title,
            cover: book.cover_url,
            status: item.status,
            createdAt: item.created_at,
            bookId: book.book_id,
            author: book.authors?.[0]?.name || '未知作者',
            author_id: book.authors?.[0]?.author_id || null,
            rate: book.ratings_count ?? 0
          }
        } catch (e) {
          console.error(`❌ 無法取得書籍 ${item.book_id}`, e)
          return null
        }
      })
    )

    this.booksByShelf = {
      All: detailedList.filter(Boolean)
    }
    this.selectedShelf = 'All'

    console.log('📚 完整書籍資料:', this.booksByShelf.All)
  } catch (err) {
    console.error('❌ 抓取 reading list 失敗:', err)
    alert('無法取得你的書籍資料，請稍後再試')
  }
}

,
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
  // 禁止刪除 "All" 書櫃
  if (name === 'All') {
    alert('「All」書櫃不能被刪除！')
    return
  }

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
  async removeBook(bookId) {
  const userId = localStorage.getItem('user_id')
  if (!userId) {
    alert('請先登入')
    return
  }

  const list = this.booksByShelf[this.selectedShelf]
  const book = list.find(b => b.id === bookId)
  if (!book) {
    alert('找不到要刪除的書籍')
    return
  }

  const itemId = book.id  // 注意：這是 reading-list 的 item_id，不是書本的 book_id
  const deleteUrl = `http://localhost:8080/users/${userId}/reading-list/${itemId}`

  try {
    // ✅ 印出實際 DELETE 請求的 URL
    console.log('🛰️ 發送 DELETE 請求:', deleteUrl)

    const response = await axios.delete(deleteUrl)

    if (response.status === 200) {
      this.booksByShelf[this.selectedShelf] = list.filter(b => b.id !== bookId)
      console.log('✅ 書籍已刪除')
    } else {
      alert('刪除失敗，請稍後再試')
    }
  } catch (err) {
    console.error('❌ DELETE 發生錯誤:', err)
    alert('刪除時發生錯誤')
  }
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
