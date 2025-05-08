<!-- components/ChooseBook.vue -->
<template>
  <div class="choose-book">
    <h3>Choose a book to add</h3>
    <ul>
      <li
        v-for="book in books"
        :key="book.id"
        @click="$emit('choose', book)"
      >
        {{ book.title }} - {{ book.author }}
      </li>
    </ul>
    <button @click="$emit('cancel')">Cancel</button>
  </div>
</template>

<script setup>
defineProps(['books'])
defineEmits(['choose', 'cancel'])
</script>

<style scoped>
.choose-book {
  background: white;
  padding: 1rem;
  border: 1px solid #ccc;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  max-height: 400px;
  overflow-y: auto;
}
.choose-book ul {
  list-style: none;
  padding: 0;
}
.choose-book li {
  padding: 0.5rem;
  cursor: pointer;
}
.choose-book li:hover {
  background-color: #f0f0f0;
}
</style>
