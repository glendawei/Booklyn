<template>
  <div class="sidebar">
    <h3>Bookshelves</h3>
    <ul>
      <li
        v-for="(count, name) in counts"
        :key="name"
        class="shelf-item"
      >
        <span @click="$emit('select', name)">
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
  background-color: #f5f5f5;
  padding: 20px;
}
.shelf-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin: 8px 0;
}
.remove-btn {
  background: none;
  border: none;
  color: rgb(0, 0, 0);
  cursor: pointer;
}
.add-btn {
  margin-top: 20px;
  width: 100%;
  padding: 8px;
  background-color: #ccc;
  font-weight: bold;
  border: none;
}
</style>
