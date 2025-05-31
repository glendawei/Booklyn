// scripts/generateDataFiles.js
import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'
import OpenAI from 'openai'
import { getAllBooks } from './api/books.js'
import { getAuthor } from './api/authors.js'

// Dynamically obtain __dirname (because type: "module")
const __dirname = path.dirname(fileURLToPath(import.meta.url))

// Output directory
const outputDir = path.resolve(__dirname, './data')

if (!fs.existsSync(outputDir)) {
  fs.mkdirSync(outputDir, { recursive: true })
}

// Initialize OpenAI Client (make sure OPENAI_API_KEY is set in environment variables)
const openai = new OpenAI({
  apiKey: process.env.OPENAI_API_KEY,
})

function slugify(name) {
  return name
    .toLowerCase()
    .replace(/\./g, '')           // Remove all dots `.`
    .replace(/[^a-z0-9\s-]/g, '') // Remove other unwanted symbols (optional)
    .trim()
    .replace(/\s+/g, '-')         // Replace spaces with hyphens
}

// Helper: Combine all reviews into a prompt, send to OpenAI, and return AI summary
async function generateReviewSummary(reviews) {
  // If there are no reviews, return an empty string
  if (!Array.isArray(reviews) || reviews.length === 0) {
    return ''
  }

  // Extract all review comments
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

    const aiSummary = completion.choices[0].message.content.trim()
    return aiSummary
  } catch (err) {
    console.error('OpenAI API error:', err)
    return ''  // Return empty string if an error occurs
  }
}

async function generate() {
  console.log('Fetching book data...')
  const booksGet = await getAllBooks()
  console.log(booksGet.length, 'book records retrieved')

  const booksByCategory = {}
  const authors = {}

  // Process each book
  for (const book of booksGet) {
    // show progress
    if (book.book_id % 100 === 0) {
      console.log(`Processing book ID ${book.book_id}...`)
    }

    // 1. Process reviews and generate a random aiRating
    let reviews = []
    if (Array.isArray(book.reviews) && book.reviews.length > 0) {
      reviews = book.reviews.map(review => ({
        reviewer: review.profile_name,
        comment: review.content,
        rating: review.rating,
        aiRating: parseFloat(Math.random().toFixed(2)), // Random number between 0 and 1 with two decimal places
      }))
    } else {
      reviews = []
    }

    // 2. Generate AI summary (await OpenAI response)
    const summaryAi = await generateReviewSummary(reviews)
    // 3. Add this book to its categories
    for (const category of book.categories) {
      if (!(category in booksByCategory)) {
        booksByCategory[category] = []
      }
      booksByCategory[category].push({
        id: book.book_id,
        title: book.title,
        author: book.authors?.[0]?.name || 'Unknown',
        rate: `${(Math.random() * 5).toFixed(1)}/5`, // Random rating between 0.0 and 5.0
        description: book.description,
        cover: book.cover_url,
        reviews: reviews,
        summary_ai: summaryAi,        // New AI summary field
      })
    }

    // 4. Process author data (same as before)
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

  // 5. Write results to files
  fs.writeFileSync(
    path.join(outputDir, 'mockBookshelves.js'),
    `export const bookshelves = ${JSON.stringify(booksByCategory, null, 2)};`
  )

  fs.writeFileSync(
    path.join(outputDir, 'authors.js'),
    `export const authors = ${JSON.stringify(authors, null, 2)};`
  )

  console.log('Data has been written to frontend/src/data/, including summary_ai fields')
}

generate()
