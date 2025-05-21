import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/HomePage.vue'
import MyBook from '../views/MyBooks.vue'
import Community from '../views/Community.vue'
import Category from '../views/Category.vue' // 修正這一行
import BookDetail from '../views/BookDetail.vue'
import Profile from '../views/Profile.vue' //作者公開頁
import Login from '../views/Login.vue'
import Setting from '../views/Setting.vue' 

import Signin from '../views/Signin.vue'
import SigninInterestPage from '@/views/SigninInterestPage.vue'
import ProfileLayout from '../layouts/ProfileLayout.vue'
import PersonalInfo from '../views/PersonalInfo.vue'
import BookPreference from '../views/BookPreference.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: Home,
    },
    { path: '/MyBooks', component: MyBook },
    { path: '/Login', component: Login },
    { path: '/Community', component: Community },
    { path: '/Category', component: Category },
    { path: '/Setting', component: Setting },
    { path: '/Signin', component: Signin },
    {
      path: '/BookDetail/:id',
      name: 'BookDetail',
      component: BookDetail
    },
    {
      path: '/profile-settings',
      component: ProfileLayout,
      children: [
        {
          path: 'personal-info',
          name: 'PersonalInfo',
          component: PersonalInfo
        },
        {
          path: 'book-preference',
          name: 'BookPreference',
          component: BookPreference
        },
        {
          path: 'settings',
          name: 'Setting',
          component: Setting
        }
      ]
    },
    {
      path: '/Profile/:id',
      name: 'Profile',
      component: Profile
    },
    { path: '/interests',    
      name: 'Interests',  
      component: SigninInterestPage },
    {
    path: '/review-detail',
    name: 'ReviewDetail',
    component: () => import('@/views/ReviewDetail.vue')
    }
    
  ],
})

export default router
