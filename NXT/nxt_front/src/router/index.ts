import { createRouter, createWebHistory } from 'vue-router'
import ai_routers from './mdl/ai_router'
import sys_routers from './mdl/sys_router'
import blk_routers from './mdl/blk_router'
import voice_routers from './mdl/voice_router'

const routes = sys_routers.concat(ai_routers, blk_routers, voice_routers)

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'lg',
      component: () => import('../views/lg.vue')
    },
    {
      path: '/main',
      name: 'main',
      component: () => import('../views/main.vue'),
      children: routes
    }
  ]
})

export default router
