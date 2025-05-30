import api from './client'

// 登录
export function login(credentials) {
  // credentials = { email, password }
  return api.post('/login', credentials).then(res => res.data)
}

// 注册
export function signup(payload) {
  // payload = { name, email, password, … }
  return api.post('/signup', payload).then(res => res.data)
}
