<!-- Setting.vue -->
<template>
    <div>
      <h1>Setting Page</h1>
      <button @click="logoutFake">Logout</button>
    </div>
  </template>
  
  <script setup>
  import { useRouter } from 'vue-router'
  
  const router = useRouter()
  
  function logoutFake() {
    localStorage.removeItem('loggedIn')
    router.push('/Login') // 登出後回登入頁
  }
  </script>
  