const ai_routers = [
  {
    path: '/sys/home',
    name: 'sys_home',
    component: () => import('../../views/home.vue'),
    meta: {
      keepAlive: false
    }
  },
  {
    path: '/sys/user',
    name: 'sys_user',
    component: () => import('../../views/sys/user.vue'),
    meta: {
      keepAlive: true
    }
  },
  {
    path: '/sys/employee',
    name: 'sys_employee',
    component: () => import('../../views/sys/employee.vue'),
    meta: {
      keepAlive: false
    }
  },
  {
    path: '/sys/menu',
    name: 'sys_menu',
    component: () => import('../../views/sys/menu.vue'),
    meta: {
      keepAlive: true
    }
  },
  {
    path: '/sys/permission',
    name: 'sys_permission',
    component: () => import('../../views/sys/permission.vue'),
    meta: {
      keepAlive: true
    }
  },
  {
    path: '/sys/setting',
    name: 'sys_setting',
    component: () => import('../../views/sys/setting.vue'),
    meta: {
      keepAlive: true
    }
  },
  {
    path: '/sys/org',
    name: 'sys_org',
    component: () => import('../../views/sys/org.vue'),
    meta: {
      keepAlive: true
    }
  },
  {
    path: '/sys/loginlog',
    name: 'sys_loginlog',
    component: () => import('../../views/sys/lglog.vue'),
    meta: {
      keepAlive: true
    }
  },
  {
    path: '/sys/tsklog',
    name: 'sys_tsklog',
    component: () => import('../../views/sys/tsklog.vue'),
    meta: {
      keepAlive: true
    }
  }
]

export default ai_routers
