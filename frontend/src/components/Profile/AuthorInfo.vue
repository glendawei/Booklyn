<template>
  <div class="author-info">
    <img :src="author.avatar" class="avatar" />
    <h2 class="name">{{ author.name }}</h2>
   <p v-if="author.created_at">Created at: {{ new Date(author.created_at).toLocaleString() }}</p>
<p v-else>Created at: Unknown</p>

  </div>
</template>

<script setup>
defineProps(['author'])
</script>

<style scoped>
.author-info {
  background-color: #eedaad;
  width: 100%;
  text-align: center;         /* Center text and inline content */
  padding: 1.5rem;
  border: none;               /* Remove side border if not needed */
}

.avatar {
  width: 96px;
  height: 96px;
  border-radius: 50%;
  margin-bottom: 0.5rem;
  display: block;
  margin-left: auto;
  margin-right: auto;         /* Center the image block */
  object-fit: cover;          /* Ensures the image covers the circle area without distortion */
  object-position: center;    /* Centers the image */
}

.name {
  font-size: 1.25rem;
  font-weight: bold;
  margin-top: 0.5rem;
}

.meta {
  font-size: 0.875rem;
  color: #666;
  margin-top: 0.25rem;
}

.description {
  font-size: 0.875rem;
  color: #363535;
  margin-top: 1rem;
}
</style>
