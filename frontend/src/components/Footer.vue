<template>
  <footer class="footer w-full">
    <div class="footer-content">
      <div class="copyright">
        © 2025 Booklyn. All rights reserved.
      </div>
      <link
  rel="stylesheet"
  href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.0/css/all.min.css"
/>
      <div class="social-icons">
        <a href="#"><i class="fab fa-x-twitter"></i></a>
        <a href="#"><i class="fab fa-instagram"></i></a>
        <a href="#"><i class="fab fa-youtube"></i></a>
        <a href="#"><i class="fab fa-linkedin"></i></a>
      </div>
    </div>
  </footer>
</template>

<style scoped>
.footer {
  background-color: #152a24; /* 深綠色 */
  color: #9a8644;            /* 棕色 */
  padding: 1.5rem 2rem;
  width: 100%;
  box-sizing: border-box;
}

.footer-content {
  display: flex;
  justify-content: space-between; /* 左右排列 */
  align-items: center;
  max-width: 1200px;
  margin: 0 auto;
}

.social-icons {
  display: flex;
  gap: 1rem;
}

.social-icons a {
  color: #9a8644;
  font-size: 1.2rem;
  transition: color 0.2s;
}

.social-icons a:hover {
  color: #ffffff;
}

</style>
