<template>
  <div class="register-container">
    <div class="form-box">
      <h1>Register</h1>

      <div class="form-group">
        <label for="name">User name</label>
        <input
          id="name"
          v-model="name"
          type="text"
          placeholder="Enter your name"
          required
        />
      </div>

      <div class="form-group">
        <label for="email">Email</label>
        <input
          id="email"
          v-model="email"
          type="email"
          placeholder="Enter your email"
          required
        />
      </div>

      <div class="form-group">
        <label for="password">Password</label>
        <input
          id="password"
          v-model="password"
          type="password"
          placeholder="Enter your password"
          required
        />
      </div>

      <div class="form-group">
        <label for="confirm">Confirm your password</label>
        <input
          id="confirm"
          v-model="confirm"
          type="password"
          placeholder="Confirm your password"
          required
        />
      </div>

      <div class="form-group terms">
        <input
          id="terms"
          v-model="accepted"
          type="checkbox"
          required
        />
        <label for="terms">
          I accept terms and conditions & privacy policy
        </label>
      </div>

      <button @click="onRegister">Register</button>

      <p class="link" @click="goToLogin">
        Already have an account? <span>Login in</span>
      </p>

      <p v-if="errorMessage" class="error">{{ errorMessage }}</p>
      <p v-if="successMessage" class="success">{{ successMessage }}</p>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import axios from 'axios'

const router = useRouter()

const name     = ref('')
const email    = ref('')
const password = ref('')
const confirm  = ref('')
const accepted = ref(false)

const errorMessage   = ref('')
const successMessage = ref('')

// ⬇️ 這裡請確認和你後端一致，如果用 VITE_API_BASE_URL 更好
const baseURL = 'http://localhost:8080'

async function onRegister() {
  if (password.value !== confirm.value) {
    errorMessage.value = '密碼與確認密碼不符'
    successMessage.value = ''
    return
  }

  if (!accepted.value) {
    errorMessage.value = '請先同意服務條款與隱私政策'
    successMessage.value = ''
    return
  }


  try {
    const response = await axios.post(`${baseURL}/signup`, {
      display_name: name.value,
      email: email.value,
      password_hash: password.value,
      role: 'reader',         // 預設註冊為一般使用者
      bio: "a",
      avatar: "https://example.com/avatar.png",
      website: "https://example.com"
    })

    successMessage.value = '註冊成功！'
    errorMessage.value = ''

    // 儲存登入狀態（這裡你可以儲存 user 或 token）
    localStorage.setItem('loggedIn', 'true')
    localStorage.setItem('user', JSON.stringify(response.data))
    localStorage.setItem('user_id', response.data.user_id)
    console.log('✅ user_id 是：', response.data.user_id)

    setTimeout(() => router.push('/interests'), 1000)
  } catch (err) {
    if (err.response?.status === 409) {
      errorMessage.value = '該 Email 已註冊過，請登入或換個 Email'
    } else {
      errorMessage.value = '註冊失敗，請稍後再試'
    }
    successMessage.value = ''
  }

  if (!accepted.value) {
    errorMessage.value = '請同意條款與隱私權政策'
    successMessage.value = ''
    return
  }

  // 檢查是否已有相同 email
  let users = JSON.parse(localStorage.getItem('users')) || []
  const duplicate = users.find(u => u.email === email.value)

  if (duplicate) {
    errorMessage.value = '該 Email 已註冊，請改用其他 Email'
    successMessage.value = ''
    return
  }

  // 建立新使用者資料
  const newUser = {
    name: name.value,
    email: email.value,
    password: password.value,
    preference: [] 
  }

  users.push(newUser)
  localStorage.setItem('users', JSON.stringify(users))

  // 登入狀態 & 儲存目前使用者 email
  localStorage.setItem('loggedIn', 'true')
  localStorage.setItem('currentUser', email.value)

  errorMessage.value = ''
  successMessage.value = '註冊成功！'

  setTimeout(() => {
    router.push('/interests')  // 下一頁是選興趣
  }, 800)

}

function goToLogin() {
  router.push('/login')
}
</script>

<style scoped>
:root, body, html {
  overflow-x: hidden;
  margin: 0;
  padding: 0;
}

.register-container {
  width: 100vw;
  height: calc(100vh - 80px - 60px);
  display: flex;
  justify-content: center;
  align-items: center;
  background: #fefae0;
  box-sizing: border-box;
}

.form-box {
  width: 400px;
  background: #4b610c;            
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 4px 12px rgba(0,0,0,0.1);
  color: rgb(238, 223, 196);      
  font-family: Arial, sans-serif;
}

.form-box h1 {
  margin-bottom: 1.5rem;
  font-size: 1.8rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  margin-bottom: 1rem;
}

.form-group label {
  margin-bottom: 0.25rem;
  font-size: 0.95rem;
}

.form-group input[type="text"],
.form-group input[type="email"],
.form-group input[type="password"] {
  padding: 0.6rem;
  border: none;
  border-radius: 8px;
  background: #fff;
  font-size: 0.9rem;
}

.terms {
  flex-direction: row;
  align-items: center;
}

.terms input {
  margin-right: 0.5rem;
}

button {
  width: 100%;
  padding: 0.6rem;
  background-color: rgb(236, 221, 193); 
  color: #4b610c;                       
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: bold;
  margin-top: 0.5rem;
}

button:hover {
  background-color: #b5c965;
}

.link {
  margin-top: 1rem;
  text-align: center;
  text-decoration: underline;
  color: rgb(236, 221, 193);
  cursor: pointer;
}

.error {
  color: #ff6961;
  margin-top: 0.5rem;
  text-align: center;
}

.success {
  color: #77dd77;
  margin-top: 0.5rem;
  text-align: center;
}
</style>
