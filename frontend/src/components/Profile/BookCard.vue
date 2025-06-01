<template>
  <router-link :to="`/books/${book.book_id}`" class="book-card-link">
    <div class="book-card">
      <img :src="book.cover_url" class="book-cover" />
      <div class="book-content">
        <h4 class="book-title">{{ book.title }}</h4>
        <p class="book-body">{{ book.description }}</p>
      </div>
    </div>
  </router-link>
</template>

<script setup>
defineProps(['book'])
</script>

<style scoped>
.book-card-link {
  text-decoration: none;
  color: rgb(4, 49, 13);
  display: block;
}

.book-card {
  display: flex;
  background: white;
  padding: 1rem;
  border-radius: 0.5rem;
  margin-bottom: 1rem;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
  transition: box-shadow 0.2s ease;
}
.book-card:hover {
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
}

.book-cover {
  width: 96px;
  height: 144px;
  object-fit: cover;
}
.book-content {
  margin-left: 1rem;
}
.book-title {
  font-size: 1rem;
  font-weight: bold;
}
.book-body {
  font-size: 0.875rem;
  color: #555;
  margin-top: 0.25rem;
}
</style>
