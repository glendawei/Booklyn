<template>
  <div class="choose-book">
    <h3>Choose books to add</h3>
    <ul>
      <li v-for="book in books" :key="book.id">
        <label class="book-item">
          <input
            type="checkbox"
            :value="book"
            v-model="selectedBooks"
          />
          <img :src="book.cover" alt="Book Cover" class="book-cover" />
          <div class="book-info">
            <p class="book-title">{{ book.title }}</p>
            <p class="book-author">{{ book.author }}</p>
          </div>
        </label>
      </li>
    </ul>
    <div class="action-buttons">
      <button @click="emitSelectedBooks" class="confirm-btn">Add Selected Books</button>
      <button @click="$emit('cancel')" class="cancel-btn">Cancel</button>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const props = defineProps(['books'])
const emit = defineEmits(['choose', 'cancel'])

const selectedBooks = ref([])

function emitSelectedBooks() {
  emit('choose', selectedBooks.value)
  selectedBooks.value = []
}
</script>

<style scoped>
.choose-book {
  background: white;
  padding: 1rem;
  border: 1px solid #ccc;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  max-height: 400px;
  overflow-y: auto;
  border-radius: 0.5rem;
}

.choose-book ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.choose-book li {
  padding: 0.75rem;
  border-bottom: 1px solid #eee;
}

.book-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  cursor: pointer;
}

.book-item input[type="checkbox"] {
  transform: scale(1.2);
  margin-right: 0.5rem;
}

.book-cover {
  width: 60px;
  height: 90px;
  object-fit: cover;
  border-radius: 4px;
}

.book-info {
  flex: 1;
}

.book-title {
  font-weight: bold;
  margin: 0;
}

.book-author {
  margin: 0;
  font-size: 0.9rem;
  color: #666;
}

.action-buttons {
  display: flex;
  justify-content: space-between;
  margin-top: 1rem;
}

.confirm-btn,
.cancel-btn {
  padding: 0.5rem 1rem;
  border: none;
  color: white;
  border-radius: 4px;
  cursor: pointer;
}

.confirm-btn {
  background-color: #2f6f4e;
}
.confirm-btn:hover {
  background-color: #25543d;
}

.cancel-btn {
  background-color: #999;
}
.cancel-btn:hover {
  background-color: #666;
}
</style>
