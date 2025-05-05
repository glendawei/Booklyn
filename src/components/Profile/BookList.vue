<template>
    <div class="book-list">
      <h3 class="heading">Books</h3>
      <BookCard v-for="book in books" :key="book.title" :book="book" />
    </div>
  </template>
  
  <script setup>
  import BookCard from './BookCard.vue'
  defineProps(['books'])
  </script>
  
  <style scoped>
  .book-list {
    padding-left: 1.5rem;
    width: 100%;
  }
  .heading {
    font-size: 1.125rem;
    font-weight: 600;
    margin-bottom: 1rem;
  }
  </style>
  