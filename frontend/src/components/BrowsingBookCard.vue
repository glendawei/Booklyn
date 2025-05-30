<template>
    <router-link :to="`/books/${book.id}`" class="book-card">
        <img :src="book.cover" :alt="book.title" class="book-cover" />

        <div class="book-info">
            <h3 class="book-title">{{ book.title }}</h3>
            <p class="book-author">{{ book.author }}</p>
            <p class="book-rate">{{ book.rate }}</p>
        </div>
    </router-link>
</template>

<script setup>
defineProps({ book: Object });
</script>

<style scoped>
.book-card {
    display: block;
    width: 200px;
    text-decoration: none;
    color: inherit;
    border-radius: 0.5rem;
}

.book-card:hover {
    background-color: rgba(0, 0, 0, 0.05);
    color: inherit;
}

.book-card:visited,
.book-card:focus,
.book-card:active {
    color: inherit;
    text-decoration: none;
}

.book-cover {
    width: 180px;
    height: 250px;
    object-fit: cover;
    border-top-left-radius: 0.5rem;
    border-top-right-radius: 0.5rem;
    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
    transition: box-shadow 0.2s ease-in-out;
    display: block;
    margin: 0 auto;
}

.book-cover:hover {
    box-shadow: 0 4px 6px rgba(0, 0, 0, 0.15);
}

.book-info {
    padding: 0.5rem;
    width: 100%;
    text-align: center;
}

.book-title {
    font-weight: 600;
    font-size: 0.875rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.book-author {
    color: #4B5563;
    font-size: 0.75rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.book-rate {
    color: #D97706;
    font-size: 0.75rem;
    font-weight: 500;
    margin-top: 0.25rem;
}
</style>
