// scripts/generateDataFiles.js
import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'
import { getAllBooks } from './api/books.js'
import { getAuthor } from './api/authors.js'

// 動態取得 __dirname（因為 type: "module"）
const __dirname = path.dirname(fileURLToPath(import.meta.url))

// 輸出路徑
const outputDir = path.resolve(__dirname, './data')

if (!fs.existsSync(outputDir)) {
  fs.mkdirSync(outputDir, { recursive: true })
}

function slugify(name) {
  return name
    .toLowerCase()
    .replace(/\./g, '')        // 移除所有點 `.`
    .replace(/[^a-z0-9\s-]/g, '') // 移除其他奇怪符號（可選）
    .trim()
    .replace(/\s+/g, '-')      // 將空白轉成 -
}

async function generate() {
  console.log('正在取得資料...')
  const booksGet = await getAllBooks()
  const books = {}
  const authors = {}

  for (const book of booksGet) {
    let reviews = []
    if (book.reviews != null) {
      book.reviews.forEach(review => {
        reviews.push({
          reviewer: review.profile_name,
          comment: review.content,
          rating: review.rating,
          aiRating: 0.5
        })
      })
    } else {
      reviews = null
    }

    for (const category of book.categories) {
      if (!(category in books)) {
        books[category] = []
      }
      books[category].push({
        id: book.book_id,
        title: book.title,
        author: book.authors[0].name,
        rate: '5/5',
        description: book.description,
        cover: book.cover_url,
        reviews: reviews,
      })
    }

    const authorId = book.authors?.[0]?.author_id
    const authorName = slugify(book.authors?.[0]?.name)

    if (authorId && authorName) {
      if (!(authorName in authors)) {
        const authorData = await getAuthor(authorId)
        authors[authorName] = {
          name: book.authors[0].name,
          avatar: "https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQlJAR3O_L5LOiRFselkydSWSJdlqnpv5H9WlkTZfyN8EeYN9FO7t6v4NU3cUMaCMyCbtriWvadhKbm86xOSHmXtA",
          followers: 1000,
          works: 0,
          description: authorData.bio,
          books: []
        }
      }

      authors[authorName].books.push({
        id: book.book_id,
        title: book.title,
        image: book.cover_url,
        body: book.description
      })
      authors[authorName].works += 1
    }
  }

  fs.writeFileSync(
    path.join(outputDir, 'mockBookshelves.js'),
    `export const bookshelves = ${JSON.stringify(books, null, 2)};`
  )

  fs.writeFileSync(
    path.join(outputDir, 'authors.js'),
    `export const authors = ${JSON.stringify(authors, null, 2)};`
  )

  console.log('資料已成功寫入 frontend/src/data/')
}

generate()