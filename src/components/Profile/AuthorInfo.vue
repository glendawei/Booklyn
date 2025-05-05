<template>
    <div class="author-info">
      <img :src="author.avatar" class="avatar" />
      <h2 class="name">{{ author.name }}</h2>
      <p class="meta">Works: {{ author.works }} | Follower: {{ author.followers.toLocaleString() }}</p>
      <p class="description">{{ author.description }}</p>
    </div>
  </template>
  
  <script setup>
  defineProps(['author'])
  </script>
  
  <style scoped>
  .author-info {
    width: 100%;
    padding-right: 1.5rem;
    border-right: 1px solid #ccc;
  }
  .avatar {
    width: 96px;
    height: 96px;
    border-radius: 50%;
    margin-bottom: 0.5rem;
  }
  .name {
    font-size: 1.25rem;
    font-weight: bold;
  }
  .meta {
    font-size: 0.875rem;
    color: #666;
    margin-top: 0.25rem;
  }
  .description {
    font-size: 0.875rem;
    color: #444;
    margin-top: 1rem;
  }
  </style>
  