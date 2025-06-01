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
  export default {
    data() {
      return {
        interests: [
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
      };
    },
    methods: {
      toggleInterest(name) {
        const index = this.selected.indexOf(name);
        if (index === -1) {
          this.selected.push(name);
        } else {
          this.selected.splice(index, 1);
        }
      },
      submit() 
      {
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
  