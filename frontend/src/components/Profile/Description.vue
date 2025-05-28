<template>
    <div class="description-card">
      <h2 class="title">information</h2>
      <p class="text">
        {{ author.description }}
      </p>
      
      <hr class="divider" />
  
      <div class="follow-section">
        <div class="follow-header">Following</div>
        <div class="avatars">
          <img v-for="(img, i) in author.followingAvatars" :key="i" :src="img" class="avatar" />
          <div class="more">+{{ author.moreFollowing }}</div>
        </div>
      </div>
  
     <!-- 確保你在 index.html 的 <head> 中加入這行 CDN -->
<link
  rel="stylesheet"
  href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.0/css/all.min.css"
/>

<!-- HTML 組件 -->
<div class="social-icons">
  <a v-if="author.x" :href="author.x" target="_blank" title="X (Twitter)">
        <i class="fab fa-x-twitter"></i>
      </a>
      <a v-if="author.ig" :href="author.ig" target="_blank" title="Instagram">
        <i class="fab fa-instagram"></i>
      </a>
      <a v-if="author.fb" :href="author.fb" target="_blank" title="Facebook">
        <i class="fab fa-facebook"></i>
      </a>
      <a v-if="author.youtube" :href="author.youtube" target="_blank" title="YouTube">
        <i class="fab fa-youtube"></i>
      </a>
      <a v-if="author.linkedin" :href="author.linkedin" target="_blank" title="LinkedIn">
        <i class="fab fa-linkedin"></i>
      </a>
</div>

    </div>
  </template>
  
  <script>
export default {
  props: ['author']
}
</script>

  
  <style scoped>
  .description-card {
    background: #def4d4;
    padding: 1.5rem;
    border-radius: 8px;
    max-width: 500px;
    margin: 0 auto;
    font-family: system-ui, sans-serif;
  }
  
  .title {
    font-size: 1.5rem;
    font-weight: 500;
    margin-bottom: 1rem;
    text-transform: lowercase;
  }
  
  .text {
    font-size: 0.95rem;
    color: #333;
    line-height: 1.6;
  }
  
  .divider {
    margin: 1.25rem 0;
    border: none;
    border-top: 1px solid #aaa;
  }
  
  .follow-section {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
  }
  
  .follow-header {
    font-weight: 500;
    margin-bottom: 0.5rem;
  }
  
  .avatars {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  
  .avatar {
    width: 32px;
    height: 32px;
    border-radius: 50%;
  }
  
  .more {
    font-size: 0.85rem;
    color: #444;
    background: #ddd;
    padding: 0 0.5rem;
    border-radius: 12px;
  }
  
  .social-icons {
    display: flex;
    gap: 1rem;
    margin-top: 1rem;
    font-size: 1.2rem;
    color: #444;
  }
  .social-icons {
  display: flex;
  gap: 1rem;
  margin-top: 0.5rem;
}

.social-icons a {
  font-size: 1.5rem;
  color: #eba225;
  transition: color 0.2s;
}

.social-icons a:hover {
  color: #1eaf99;
}

  </style>
  