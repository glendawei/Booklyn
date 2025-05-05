<template>
  <nav class="navbar">
    <!-- 左側 Logo，點擊跳到 Home -->
    <router-link to="/" class="logo">Read</router-link>

    <!-- 中間 Tabs -->
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

    <!-- 右側 Search + Profile -->
    <div class="actions">
      <div class="search-box">
        <svg class="icon icon-search" viewBox="0 0 24 24">
          <path
            d="M10 2a8 8 0 015.292 13.708l4.5 4.5-1.414 1.414-4.5-4.5A8 8 0 1110 2zm0 2a6 6 0 100 12 6 6 0 000-12z"
          />
        </svg>
        <input
          v-model="query"
          type="text"
          placeholder="Value"
        />
      </div>
      <svg class="icon icon-user" viewBox="0 0 24 24">
        <path
          d="M12 12a5 5 0 100-10 5 5 0 000 10zm0 2c-3.866 0-7 1.343-7 4v2h14v-2c0-2.657-3.134-4-7-4z"
        />
      </svg>
    </div>
  </nav>
</template>

<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

// 為每個 tab 定義名稱與路徑
const tabs = [
  { name: 'My books',  path: '/MyBooks' },
  { name: 'Community', path: '/Community' },
  { name: 'Category',  path: '/Category' },
]

const selected = ref(tabs[0].name)
const query = ref('')

function select(tab) {
  selected.value = tab.name
  router.push(tab.path)
}
</script>

<style scoped>
.navbar {
  position: fixed;
  top: 0; left: 0; right: 0;
  z-index: 1000;
  display: flex;
  align-items: center;
  flex-wrap: wrap;
  padding: 0 16px;
  background: #000;
  color: #fff;
  height: 56px;
}

/* Logo 改成 router-link */
.logo {
  display: inline-block;
  background: #333;
  padding: 0 16px;
  font-size: 1.2rem;
  font-weight: bold;
  line-height: 56px;
  color: #fff;
  text-decoration: none;
  white-space: nowrap;
}

.tabs {
  display: flex;
  gap: 12px;
  margin: 0 24px;
  flex: 1;
  min-width: 0;
  flex-wrap: wrap;
}

.tab {
  padding: 6px 12px;
  border: 1px solid #fff;
  border-radius: 4px;
  cursor: pointer;
  white-space: nowrap;
  transition: background-color .2s, color .2s;
}
.tab.active {
  background: #fff;
  color: #000;
}

.actions {
  display: flex;
  align-items: center;
  gap: 16px;
  flex-shrink: 0;
}

.search-box {
  position: relative;
  display: flex;
  align-items: center;
  background: #fff;
  border-radius: 24px;
  padding: 4px 12px;
}
.search-box input {
  border: none;
  outline: none;
  padding: 6px 8px 6px 28px;
  font-size: 0.9rem;
  border-radius: 24px;
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
  fill: #fff;
  cursor: pointer;
}

@media (max-width: 640px) {
  .navbar {
    padding: 0 8px;
    height: auto;
    row-gap: 8px;
  }
  .tabs {
    margin: 8px 0;
    justify-content: center;
  }
  .search-box input {
    min-width: 100px;
  }
}
</style>
