<template>
  <div class="forum-container">
    <h1 class="forum-title">Community Forum</h1>

    <div v-for="post in posts" :key="post.id" class="forum-post">
      <router-link :to="`/PostDetail/${post.id}`" class="post-link">
        <h2 class="post-title">{{ post.title }}</h2>
      </router-link>
      <div class="post-meta">
        <span class="post-author">by {{ post.author }}</span>
        <span class="post-date">{{ post.date }}</span>
      </div>
      <p class="post-preview">{{ post.preview }}</p>
      <div class="post-footer">
        <span class="comments-count">{{ post.comments }} comments</span>
      </div>
    </div>
  </div>
</template>

<script setup>
const posts = [
  {
    id: 1,
    title: 'Best Fantasy Books to Start With?',
    author: 'JaneDoe123',
    date: 'May 17, 2025',
    preview: 'Hey everyone! I’m new to fantasy and wondering where to start...',
    comments: 12,
  },
  {
    id: 2,
    title: 'Discussion: The Ending of Mistborn',
    author: 'BookLover22',
    date: 'May 15, 2025',
    preview: 'Just finished The Final Empire and OMG... let’s talk about that twist!',
    comments: 34,
  },
  {
    id: 3,
    title: 'Locke Lamora vs Kvothe – Who Wins?',
    author: 'FantasyFiend',
    date: 'May 10, 2025',
    preview: 'Imagine a face-off between the Gentleman Bastard and the Arcanist...',
    comments: 21,
  },
];
</script>

<style scoped>
.forum-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem 1rem;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}

.forum-title {
  font-size: 2rem;
  font-weight: bold;
  margin-bottom: 1.5rem;
  text-align: center;
  color: #333;
}

.forum-post {
  background-color: #f9f9f9;
  border-radius: 12px;
  padding: 1rem;
  margin-bottom: 1.5rem;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.08);
  transition: box-shadow 0.2s ease-in-out;
}

.forum-post:hover {
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.12);
}

.post-link {
  text-decoration: none;
  color: #333;
}

.post-title {
  font-size: 1.25rem;
  font-weight: 600;
  margin: 0 0 0.5rem;
}

.post-meta {
  font-size: 0.875rem;
  color: #666;
  margin-bottom: 0.5rem;
}

.post-meta span {
  margin-right: 1rem;
}

.post-preview {
  font-size: 0.95rem;
  line-height: 1.4;
  color: #444;
  margin-bottom: 0.75rem;
}

.post-footer {
  font-size: 0.85rem;
  color: #888;
  text-align: right;
}
</style>
