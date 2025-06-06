<template>
    <div class="interest-page">
      <h2>Welcome to booklyn! Let us get to know you first.</h2>
      <div class="interest-grid">
        <div 
          v-for="interest in interests" 
          :key="interest.name"
          :class="['interest-item', selected.includes(interest.name) ? 'selected' : '']"
          @click="toggleInterest(interest.name)"
        >
          <span class="icon">{{ interest.icon }}</span>
          <span class="label">{{ interest.name }}</span>
        </div>
      </div>
      <button class="submit-btn" @click="submit">Get started</button>
    </div>
  </template>
  <script>
import axios from 'axios';

export default {
  data() {
    return {
      interests: [],
      selected: [],
      emojiMap: [
        '📖', '✝️', '🏛️', '🧬', '🏀', '🧘‍♀️', '📚', '💰', '🧒', '🎨', '🌍', '📐', '🔬'
      ]
    };
  },
  mounted() {
    this.fetchCategoriesFromBooks();
  },
  methods: {
    async fetchCategoriesFromBooks() {
      try {
        const response = await axios.get('http://localhost:8080/books'); // 或你的實際 API URL
        const books = response.data;

        const categorySet = new Set();
        books.forEach(book => {
          (book.categories || []).forEach(cat => categorySet.add(cat));
        });

        const allCategories = Array.from(categorySet);

        // 將類別轉成 { name, icon } 格式，icon 可依照類別指定或隨機
        this.interests = allCategories.map((name, index) => ({
          name,
          icon: this.emojiMap[index % this.emojiMap.length]
        }));
      } catch (error) {
        console.error('無法取得書籍類別：', error);
      }
    },
    toggleInterest(name) {
      const index = this.selected.indexOf(name);
      if (index === -1) {
        this.selected.push(name);
      } else {
        this.selected.splice(index, 1);
      }
    },
    submit() {
      const currentUserEmail = localStorage.getItem('currentUser');
      if (!currentUserEmail) {
        alert('請先登入！');
        return;
      }

      let users = JSON.parse(localStorage.getItem('users')) || [];
      const userIndex = users.findIndex(u => u.email === currentUserEmail);

      if (userIndex !== -1) {
        users[userIndex].preference = [...this.selected];
      } else {
        users.push({
          email: currentUserEmail,
          password: '',
          preference: [...this.selected]
        });
      }

      localStorage.setItem('users', JSON.stringify(users));
      this.$router.push('/profile-settings');
    }
  }
};
</script>

  
  <style scoped>
  .interest-page {
    max-width: 800px;
    margin: 60px auto;
    text-align: center;
    padding: 20px;
  }
  
  h2 {
    margin-bottom: 30px;
    font-size: 28px;
    font-weight: bold;
    color: #333;
  }
  
  .interest-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(150px, 1fr));
    gap: 15px;
    margin-bottom: 40px;
  }
  
  .interest-item {
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    padding: 20px;
    border: 2px solid #e0e0e0;
    border-radius: 15px;
    cursor: pointer;
    transition: all 0.3s;
    background-color: #fafafa;
  }
  
  .interest-item:hover {
    border-color: #606C38;
  }
  
  .interest-item.selected {
    background-color: #606C38;
    color: white;
    border-color: #606C38;
  }
  
  .icon {
    font-size: 24px;
    margin-bottom: 8px;
  }
  
  .label {
    font-size: 16px;
    font-weight: 500;
  }
  
  .submit-btn {
    background-color: #DDA15E;
    color: white;
    padding: 14px 40px;
    border: none;
    border-radius: 8px;
    font-size: 16px;
    cursor: pointer;
    transition: background-color 0.3s;
  }
  
  .submit-btn:hover {
    background-color: #BC6C25;
  }
  </style>
  