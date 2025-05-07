<template>
  <nav :class="['navbar', { 'navbar-scrolled': isScrolled }]">
    <router-link to="/" class="logo">Booklyn</router-link>

    <div class="tabs">
      <div
        v-for="tab in tabs"
        :key="tab.name"
        :class="['tab', { active: tab.name === selected }]"
        @click="select(tab)"
      >
        {{ tab.name }}
      </div>
    </div>

    <div class="actions">
      <div class="search-box">
        <svg class="icon icon-search" viewBox="0 0 24 24">
          <path d="M10 2a8 8 0 015.292 13.708l4.5 4.5-1.414 1.414-4.5-4.5A8 8 0 1110 2zm0 2a6 6 0 100 12 6 6 0 000-12z"/>
        </svg>
        <input v-model="query" type="text" placeholder="Search..." />
      </div>
      <svg class="icon icon-user" viewBox="0 0 24 24">
        <path d="M12 12a5 5 0 100-10 5 5 0 000 10zm0 2c-3.866 0-7 1.343-7 4v2h14v-2c0-2.657-3.134-4-7-4z"/>
      </svg>
    </div>
  </nav>
</template>

<script setup>
import { computed, ref, onMounted, onUnmounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

const tabs = [
  { name: 'My books', path: '/MyBooks' },
  { name: 'Community', path: '/Community' },
  { name: 'Category', path: '/Category' },
]

const selected = computed(() => {
  const currentTab = tabs.find(tab => tab.path === route.path)
  return currentTab ? currentTab.name : null
})

const query = ref('')
const isScrolled = ref(false)

function select(tab) {
  router.push(tab.path)
}

function handleScroll() {
  isScrolled.value = window.scrollY > 20
}

onMounted(() => {
  window.addEventListener('scroll', handleScroll)
})
onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
})
</script>

<style scoped>
:root {
  --main-dark: #283618;
  --accent: #606C38;
  --text-light: #fefae0;
}

.navbar {
  position: fixed;
  top: 0; left: 0; right: 0;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: space-between;
  flex-wrap: wrap;
  padding: 0 24px;
  background: #283618;
  color: #fefae0;
  height: 80px;
  transition: all 0.3s ease;
  box-shadow: none;
}
.navbar-scrolled {
  height: 66px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.3);
}

.logo {
  font-size: 1.5rem;
  font-weight: bold;
  color: #fefae0;
  text-decoration: none;
  padding: 8px 16px;
  border-radius: 8px;
  background-color: #606C38;
  transition: background-color 0.3s ease;
}
.logo:hover {
  background-color: #4e562e;
}

.tabs {
  display: flex;
  gap: 12px;
  flex: 1;
  justify-content: right;
  flex-wrap: wrap;
  padding: 0px 15px;
}

.tab {
  padding: 6px 16px;
  /*border: 2px solid #fefae0;*/
  border-radius: 999px;
  cursor: pointer;
  transition: background-color 0.3s ease, color 0.3s ease;
}
.tab:hover {
  background-color: #606C38;
  color: #fefae0;
}
.tab.active {
  background: #fefae0;
  color: #283618;
  font-weight: bold;
}

.actions {
  display: flex;
  align-items: center;
  gap: 16px;
}

.search-box {
  position: relative;
  display: flex;
  align-items: center;
  background: #fff;
  border-radius: 999px;
  padding: 4px 12px;
}
.search-box input {
  border: none;
  outline: none;
  padding: 6px 8px 6px 28px;
  font-size: 0.9rem;
  border-radius: 999px;
  min-width: 140px;
}
.icon-search {
  position: absolute;
  left: 8px;
  width: 16px;
  height: 16px;
  fill: #888;
}
.icon-user {
  width: 24px;
  height: 24px;
  fill: #fefae0;
  cursor: pointer;
  transition: transform 0.2s ease;
}
.icon-user:hover {
  transform: scale(1.1);
}

@media (max-width: 640px) {
  .navbar {
    padding: 0 12px;
    flex-direction: column;
    height: auto;
  }
  .tabs {
    margin: 8px 0;
  }
}
</style>
