<template>
  <div class="container">
    <!-- 叉叉按鈕，回到首頁 -->
    <router-link to="/" class="close-btn">✕</router-link>

<aside class="sidebar">
  <ul>
    <li :class="{ active: $route.path === '/profile-settings/personal-info' }">
      <router-link to="/profile-settings/personal-info">Personal Info</router-link>
    </li>
    <li :class="{ active: $route.path === '/profile-settings/book-preference' }">
      <router-link to="/profile-settings/book-preference">Revise Book Preference</router-link>
    </li>
    <li>
      <button class="logout-btn" @click="logout">Logout</button>
    </li>
  </ul>
</aside>


    <main class="main">
      <router-view />
    </main>
  </div>
</template>

<script>
export default {
  methods: {
    logout() {
      localStorage.removeItem('loggedIn')
      this.$router.push('/login');
    }
  }
}
</script>

<style scoped>
.container {
  position: relative;    
  display: flex;
  min-height: 100vh;
}

.close-btn {
  position: absolute;
  top: 16px;
  right: 16px;
  font-size: 24px;
  color: #444;
  text-decoration: none;
  background: rgba(255,255,255,0.8);
  width: 32px;
  height: 32px;
  line-height: 32px;
  text-align: center;
  border-radius: 50%;
  transition: background 0.2s, color 0.2s;
}
.close-btn:hover {
  background: #606C38;
  color: #FEFAE0;
}

.sidebar {
  width: 220px;
  padding: 20px;
  background: #FEFAE0;
  border-right: 1px solid #eee;
  position: relative;
}

.sidebar ul {
  list-style: none;
  padding: 0;
}

.sidebar li {
  margin: 16px 0;
}

.sidebar a {
  text-decoration: none;
  color: #444;
}

.sidebar .active a {
  color: #606C38;
  font-weight: bold;
}

.logout-btn {
  padding: 8px 16px;
  background-color: #BC6C25;
  color: #FEFAE0;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: bold;
  transition: background 0.2s;
}

.logout-btn:hover {
  background-color: #A55D1E;
}

.main {
  flex: 1;
  padding: 40px;
  background-color: #99a56e;
}
</style>
