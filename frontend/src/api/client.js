import axios from 'axios'

const isNode = typeof window === 'undefined'

const api = axios.create({
  baseURL: isNode
    ? process.env.VITE_API_BASE_URL || 'http://localhost:8080'
    : import.meta.env.VITE_API_BASE_URL,
  timeout: 60000,
  withCredentials: true,
})

export default api
