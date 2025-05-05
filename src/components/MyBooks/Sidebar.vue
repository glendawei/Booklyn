<template>
  <div class="sidebar">
    <h3>Bookshelves</h3>
    <ul>
      <li
        v-for="(count, name) in counts"
        :key="name"
        class="shelf-item"
      >
        <span class="shelf-name" @click="$emit('select', name)">
          {{ name }} ({{ count }})
        </span>
        <button class="remove-btn" @click="$emit('remove-shelf', name)">X</button>
      </li>
    </ul>
    <button class="add-btn" @click="addShelf">+ Add Shelf</button>
  </div>
</template>

<script>
export default {
  props: {
    counts: Object
  },
  methods: {
    addShelf() {
      const name = prompt('請輸入新書櫃名稱：')
      if (name && !this.counts[name]) {
        this.$emit('add-shelf', name)
      }
    }
  }
}
</script>

<style scoped>
.sidebar {
  width: 180px;
  background-color: #f8ebdc; /* 書櫥背景色 */
  padding: 20px;
}

h3 {
  margin-bottom: 12px;
  color: #1f2937;
  font-weight: 600;
  font-size: 1.1rem;
}

ul {
  padding: 0;
  margin: 0;
  list-style: none;
}

.shelf-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin: 8px 0;
  padding: 0;
}

.shelf-name {
  text-align: left;
  flex-grow: 1;
  margin-left: 0;
  cursor: pointer;
}

.remove-btn {
  background: none;
  border: none;
  color: #111827;
  font-size: 1rem;
  cursor: pointer;
  padding: 0;
}

.add-btn {
  margin-top: 20px;
  width: 100%;
  padding: 8px;
  background-color: #e6cd88;
  font-weight: bold;
  border: none;
  cursor: pointer;
}
</style>
