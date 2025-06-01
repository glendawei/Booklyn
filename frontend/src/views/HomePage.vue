<template>
  <div style="padding: 1.5rem;">
    <div v-for="(group, index) in bookGroups" :key="index" style="margin-bottom: 2.5rem;">
      <h2 style="font-weight: 700; font-size: x-large; color: #BC6C25;">
        Because You Like {{ group.genre }}
      </h2>

      <Carousel v-bind="carouselConfig" style="height: 430px;">
        <Slide v-for="book in group.books" :key="book.id" class="custom-slide">
          <div class="carousel__item">
            <router-link :to="`/books/${book.id}`">
              <img :src="book.cover" :alt="book.title"
                style="width: 200px; height: 300px; object-fit: cover; border-top-left-radius: 0.5rem; border-top-right-radius: 0.5rem;" />
            </router-link>
            <div style="padding: 0.5rem; width: 200px;">
              <h3
                style="font-weight: 600; font-size: 0.875rem; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                {{ book.title }}</h3>
              <p
                style="color: #4B5563; font-size: 0.75rem; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;">
                {{ book.author }}</p>
              <p style="color: #D97706; font-size: 0.75rem; font-weight: 500; margin-top: 0.25rem;">{{ book.rate }}</p>
            </div>
          </div>
        </Slide>

        <template #addons>
          <Navigation />
          <Pagination />
        </template>
      </Carousel>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import { bookshelves } from '@/data/mockBookshelves.js'
import 'vue3-carousel/carousel.css'
import { Carousel, Slide, Pagination, Navigation } from 'vue3-carousel'

const carouselConfig = {
  itemsToShow: 6,
  wrapAround: true,
  autoplay: 2000,
  gap: 10,
  pauseAutoplayOnHover: true,
}
const bookGroups = ref(
  Object.entries(bookshelves)
    .filter(([genre]) => genre !== 'All') // Exclude "All"
    .map(([genre, books]) => ({
      genre,
      books
    }))
)

</script>

<style scoped>
.custom-slide {
  max-width: 200px;
  flex-shrink: 0;
  border-radius: 0.5rem;
  transition: transform 0.3s, box-shadow 0.3s;
}

.custom-slide:hover {
  transform: translateY(-4px);
}

img {
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

img:hover {
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.15);
}

.scrollbar-hide::-webkit-scrollbar {
  display: none;
}

.scrollbar-hide {
  -ms-overflow-style: none;
  /* IE and Edge */
  scrollbar-width: none;
  /* Firefox */
}
</style>
