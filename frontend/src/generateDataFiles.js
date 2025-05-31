// scripts/generateDataFiles.js
import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'
import OpenAI from 'openai'
import { getAllBooks } from './api/books.js'
import { getAuthor } from './api/authors.js'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const outputDir = path.resolve(__dirname, './data')

if (!fs.existsSync(outputDir)) {
  fs.mkdirSync(outputDir, { recursive: true })
}

const openai = new OpenAI({
  apiKey: process.env.OPENAI_API_KEY,
})

function slugify(name) {
  return name
    .toLowerCase()
    .replace(/\./g, '')
    .replace(/[^a-z0-9\s-]/g, '')
    .trim()
    .replace(/\s+/g, '-')
}

async function generateReviewSummary(reviews) {
  if (!Array.isArray(reviews) || reviews.length === 0) {
    return ''
  }

  const allComments = reviews
    .map((r, idx) => `${idx + 1}. "${r.comment}"`)
    .join('\n')

  const prompt = `
Please write a concise English summary (no more than 100 words) capturing the main points and overall positive and negative feedback from the following user reviews:
${allComments}
Begin the summary:
  `.trim()

  try {
    const completion = await openai.chat.completions.create({
      model: 'gpt-3.5-turbo',
      messages: [
        { role: 'system', content: 'You are a precise review summary assistant. Please respond in clear English.' },
        { role: 'user', content: prompt },
      ],
      temperature: 0.7,
      max_tokens: 200,
    })

    return completion.choices[0].message.content.trim()
  } catch (err) {
    console.error('OpenAI API error:', err)
    return ''
  }
}

async function generate() {
  console.log('Fetching book data...')
  const booksGet = await getAllBooks()
  console.log(booksGet.length, 'book records retrieved')

  const booksByCategory = {}
  const authors = {}

  // 先建立一個專門放「所有書」的分類
  booksByCategory['All'] = []

  for (const book of booksGet) {
    if (book.book_id % 100 === 0) {
      console.log(`Processing book ID ${book.book_id}...`)
    }

    // 處理 reviews
    let reviews = []
    if (Array.isArray(book.reviews) && book.reviews.length > 0) {
      reviews = book.reviews.map(review => ({
        reviewer: review.profile_name,
        comment: review.content,
        rating: review.rating,
        aiRating: parseFloat(Math.random().toFixed(2)),
      }))
    }

    // 產生 AI summary
    const summaryAi = await generateReviewSummary(reviews)

    // 統一建立一個 bookData 物件，之後推到對應分類
    const bookData = {
      id: book.book_id,
      title: book.title,
      author: book.authors?.[0]?.name || 'Unknown',
      rate: `${(Math.random() * 5).toFixed(1)}/5`,
      description: book.description,
      cover: book.cover_url,
      reviews: reviews,
      summary_ai: summaryAi,
    }

    // 先把這本書放到 "all" 分類
    booksByCategory['All'].push(bookData)

    // 再根據每個 category，把相同 bookData 塞進去
    for (const category of book.categories) {
      if (!(category in booksByCategory)) {
        booksByCategory[category] = []
      }
      booksByCategory[category].push(bookData)
    }

    // 處理作者資料
    const authorId = book.authors?.[0]?.author_id
    const authorName = slugify(book.authors?.[0]?.name || '')
    if (authorId && authorName) {
      if (!(authorName in authors)) {
        try {
          const authorData = await getAuthor(authorId)
          authors[authorName] = {
            name: book.authors[0].name,
            avatar:
              'https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcQlJAR3O_L5LOiRFselkydSWSJdlqnpv5H9WlkTZfyN8EeYN9FO7t6v4NU3cUMaCMyCbtriWvadhKbm86xOSHmXtA',
            followers: 1000,
            works: 0,
            description: authorData.bio,
            books: [],
          }
        } catch (err) {
          console.warn(`Unable to fetch data for author ${authorId}:`, err)
          authors[authorName] = {
            name: book.authors[0].name,
            avatar: '',
            followers: 0,
            works: 0,
            description: '',
            books: [],
          }
        }
      }

      authors[authorName].books.push({
        id: book.book_id,
        title: book.title,
        image: book.cover_url,
        body: book.description,
      })
      authors[authorName].works += 1
    }
  }

  // 輸出到檔案
  fs.writeFileSync(
    path.join(outputDir, 'mockBookshelves.js'),
    `export const bookshelves = ${JSON.stringify(booksByCategory, null, 2)};`
  )

  fs.writeFileSync(
    path.join(outputDir, 'authors.js'),
    `export const authors = ${JSON.stringify(authors, null, 2)};`
  )

  console.log('Data has been written to frontend/src/data/, including summary_ai fields and "all" category')
}

generate()
