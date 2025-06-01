<template>
  <div class="book-pref-page">
    <h2>Revise your book preference</h2>
    <p>Choose your book labels:</p>

    <div class="preference-grid">
      <div
        v-for="genre in genres"
        :key="genre.name"
        :class="['genre-box', selected.includes(genre.name) ? 'selected' : '']"
        @click="toggleGenre(genre.name)"
      >
        <span class="icon">{{ genre.icon }}</span>
        <span class="name">{{ genre.name }}</span>
      </div>
    </div>

    <button class="save-btn" @click="savePreferences">Save your Preferences</button>
  </div>
</template>

<script>
export default {
  data() {
    return {
      genres: [
        { name: 'Fiction', icon: '📖' },
        { name: 'Religion', icon: '✝️' },
        { name: 'History', icon: '🏛️' },
        { name: 'Biography & Autobiography', icon: '🧬' },
        { name: 'Sports & Recreation', icon: '🏀' },
        { name: 'Body, Mind & Spirit', icon: '🧘‍♀️' },
        { name: 'Juvenile Fiction', icon: '📚' },
        { name: 'Business & Economics', icon: '💰' },
        { name: 'Juvenile Nonfiction', icon: '🧒' }
      ],
      selected: []
    }
  },
  mounted() {
    const currentUserEmail = localStorage.getItem('currentUser');
    const users = JSON.parse(localStorage.getItem('users')) || [];
    const user = users.find(u => u.email === currentUserEmail);
    if (user && user.preference) {
      this.selected = [...user.preference];
    }
  },
  methods: {
    toggleGenre(name) {
      const index = this.selected.indexOf(name);
      if (index === -1) {
        this.selected.push(name);
      } else {
        this.selected.splice(index, 1);
      }
    },
    savePreferences() {
      const currentUserEmail = localStorage.getItem('currentUser');
      if (!currentUserEmail) {
        alert('請先登入');
        return;
      }

      let users = JSON.parse(localStorage.getItem('users')) || [];
      const userIndex = users.findIndex(u => u.email === currentUserEmail);

      if (userIndex !== -1) {
        users[userIndex].preference = [...this.selected];
        localStorage.setItem('users', JSON.stringify(users));
        alert('偏好已更新為：' + this.selected.join(', '));
      } else {
        alert('找不到使用者資料');
      }
    }
  }
};
</script>

<style scoped>
.book-pref-page {
  padding: 40px;
  color: #99a56e;
  background-color: #99a56e;
  min-height: 100vh;
}

h2 {
  font-size: 28px;
  margin-bottom: 10px;
  color: #FEFAE0;
}

p {
  color: #FEFAE0;
  margin-bottom: 24px;
}

.preference-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
  gap: 20px;
  margin-bottom: 30px;
}

.genre-box {
  background-color: #FEFAE0;
  border: 2px solid transparent;
  border-radius: 12px;
  padding: 20px;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 2px 2px 5px rgba(0,0,0,0.1);
}

.genre-box:hover {
  border-color: #DDA15E;
}

.genre-box.selected {
  background-color: #283618;
  color: white;
  border-color: #BC6C25;
}

.icon {
  font-size: 24px;
  display: block;
  margin-bottom: 10px;
}

.name {
  font-size: 16px;
  font-weight: 500;
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