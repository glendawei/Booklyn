<template>
  <div class="setting-page">
    <h2>This is a setting-page</h2>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'

const router = useRouter()

function logoutFake() {
  localStorage.removeItem('loggedIn')
  router.push('/Login')
}
</script>

<style scoped>
.setting-page {
  padding: 40px;
  background-color: #99a56e;
  color: #FEFAE0;
  min-height: 100vh;
}

.setting-page h2 {
  font-size: 28px;
  margin-bottom: 10px;
  color: #FEFAE0;
}

.setting-page p {
  color: #FEFAE0;
  margin-bottom: 24px;
}

.save-btn {
  background-color: #BC6C25;
  color: white;
  padding: 12px 28px;
  font-size: 16px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: background 0.3s;
}

.save-btn:hover {
  background-color: #A95A1D;
}
</style>