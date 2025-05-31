import api from './client.js'

// 获取所有书籍，并过滤掉出现数量少于 5 次的分类，同时删除没有任何 reviews 的书籍
export async function getAllBooks() {
  // 从后端获取原始书籍列表
  const allBooks = await api.get('/books').then(res => res.data)

  // 统计每个 category 出现的次数
  const categoryCount = {}
  allBooks.forEach(book => {
    book.categories.forEach(cat => {
      categoryCount[cat] = (categoryCount[cat] || 0) + 1
    })
  })

  // 找出出现次数 >= 5 的合法分类
  const validCategories = new Set(
    Object.entries(categoryCount)
      .filter(([, count]) => count >= 5)
      .map(([cat]) => cat)
  )

  // 对每本书进行初步处理：只保留合法分类，如果分类数组变空，则丢弃；同时删除没有 reviews 或 reviews 数组为空的书籍
  const filteredBooks = allBooks
    .map(book => {
      // 如果没有 reviews 或 reviews 为空，直接返回 null
      if (!Array.isArray(book.reviews) || book.reviews.length === 0) {
        return null
      }

      // 只保留有效分类
      const filteredCats = book.categories.filter(cat => validCategories.has(cat))
      if (filteredCats.length === 0) {
        return null
      }

      return {
        ...book,
        categories: filteredCats
      }
    })
    // 删除所有被标记为 null 的书籍
    .filter(book => book !== null)

  return filteredBooks
}

// 获取单本书详情
export function getBook(bookId) {
  return api.get(`/books/${bookId}`).then(res => res.data)
}
