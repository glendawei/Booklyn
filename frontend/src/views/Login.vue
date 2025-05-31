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
  
  <script setup>
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
  