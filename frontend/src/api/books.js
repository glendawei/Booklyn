import api from './client.js'

// 获取所有书籍
export function getAllBooks() {
  return api.get('/books').then(res => res.data)
}

// 获取单本书详情
export function getBook(bookId) {
  return api.get(`/books/${bookId}`).then(res => res.data)
}
