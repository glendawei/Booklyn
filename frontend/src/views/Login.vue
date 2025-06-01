<template>
    <div class="login-container">
      <h1>Login</h1>
  
      <div class="form-group">
        <label>Email</label>
        <input v-model="email" type="email" placeholder="Enter your email" />
      </div>
  
      <div class="form-group">
        <label>Password</label>
        <input v-model="password" type="password" placeholder="Enter your password" />
      </div>
  
      <button @click="handleLogin">Login</button>
  
      <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
      <p v-if="successMessage" class="success">{{ successMessage }}</p>
  
      <p class="link" @click="goToSignup">Don't have an account? Sign up</p>
    </div>
  </template>
  
  <!-- <script setup>
  import { ref } from 'vue'
  import { useRouter } from 'vue-router'
  import { users } from '@/data/user.js'  // 匯入使用者資料
  
  const router = useRouter()
  
  const email = ref('')
  const password = ref('')
  const errorMessage = ref('')
  const successMessage = ref('')
  
  function handleLogin() {
    const user = users.find(u => u.email === email.value)
  
    if (!user) {
      errorMessage.value = 'X This account does not exist.'
      successMessage.value = ''
      return
    }
  
    if (user.password !== password.value) {
  errorMessage.value = 'X Password incorrect'
  successMessage.value = ''
  return
}
  
    // 登入成功
    localStorage.setItem('loggedIn', 'true')
    localStorage.setItem('currentUser', email.value)
    errorMessage.value = ''
    successMessage.value = ' Login successful!'
    setTimeout(() => router.push('/'), 800)
  }
  
  function goToSignup() {
    router.push('/Signin') // 假設有 Signin 頁面
  }
  </script> -->
  <script setup>
  import { ref } from 'vue'
  import { useRouter } from 'vue-router'
  import axios from 'axios'

  const router = useRouter()

  const email = ref('')
  const password = ref('')
  const errorMessage = ref('')
  const successMessage = ref('')

  // 假設你不使用雜湊（只是 plain-text 測試階段），則用 password 傳入 password_hash 欄位
  async function handleLogin() {
    try {
      const response = await axios.post('http://localhost:8080/login', {
        email: email.value,
        password_hash: password.value,  // 傳到後端的欄位名稱是 password_hash
      })

      // 登入成功，response.data 是後端傳回來的 User 結構
      successMessage.value = 'Login successful!'
      errorMessage.value = ''
      // ✅ 儲存登入狀態與使用者 ID
    localStorage.setItem('user_id', response.data.user_id)
    localStorage.setItem('loggedIn', 'true')
    console.log('✅ user_id 是：', response.data.user_id)


      // 可儲存登入資訊，例如使用者資料或 JWT（未來擴充）
      // localStorage.setItem('user', JSON.stringify(response.data))

      setTimeout(() => router.push('/'), 800)
    } catch (err) {
      if (err.response && err.response.status === 401) {
        errorMessage.value = 'X The account does not exist or the password is wrong.'
      } else if (err.response && err.response.status === 500) {
        errorMessage.value = 'X Internal server error.'
      } else {
        errorMessage.value = 'X An unexpected error occurred.'
        console.log(err)
        console.log(err.response?.status)
        console.log(err.response?.data)
      }
      successMessage.value = ''
    }
  }

  function goToSignup() {
    router.push('/Signin')
  }
  </script>
  
  <style scoped>
 .login-container {
  max-width: 400px;
  margin: 100px auto;
  padding: 2rem;
  border-radius: 12px;
  background: #4b610c;
  color: rgb(238, 223, 196); /* ⬅️ 加這行 */
}

  
  .form-group {
    margin-bottom: 1rem;
  }
  
  input {
    width: 100%;
    padding: 0.6rem;
    margin-top: 0.2rem;
    border: 1px solid #ccc;
    border-radius: 6px;
  }
  
  button {
    width: 100%;
    padding: 0.6rem;
    background-color:rgb(236, 221, 193);
    color: #4b610c;
    border: none;
    border-radius: 6px;
    cursor: pointer;
    font-weight: bold;
  }
  
  button:hover {
    background-color: #b5c965;
  }
  
  .error {
    color: red;
    margin-top: 0.5rem;
  }
  
  .success {
    color: green;
    margin-top: 0.5rem;
  }
  
  .link {
    margin-top: 1rem;
    text-decoration: underline;
    color: #b5c965;;
    cursor: pointer;
  }
  </style>
  