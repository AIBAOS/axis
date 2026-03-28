import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Backups from '../views/Backups.vue'
import Files from '../views/Files.vue'
import Storage from '../views/Storage.vue'
import Users from '../views/Users.vue'
import System from '../views/System.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/backups',
    name: 'Backups',
    component: Backups
  },
  {
    path: '/files',
    name: 'Files',
    component: Files
  },
  {
    path: '/storage',
    name: 'Storage',
    component: Storage
  },
  {
    path: '/users',
    name: 'Users',
    component: Users
  },
  {
    path: '/system',
    name: 'System',
    component: System
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
