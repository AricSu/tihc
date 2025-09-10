export const basicRoutes = [
  {
    name: 'Login',
    path: '/',
    component: () => import('@/views/login/index.vue'),
    meta: {
      title: '登录页',
      layout: 'empty',
    },
  },
  {
    name: 'Home',
    path: '/home',
    component: () => import('@/views/home/index.vue'),
    meta: {
      title: '首页',
    },
  },
  {
    name: 'Inspection',
    path: '/inspection',
    component: () => import('@/views/inspection-access/index.vue'),
    meta: {
      title: '巡检报告',
    },
  },
  {
    name: 'SQLEditor',
    path: '/sql-editor',
    component: () => import('@/views/sql-editor/index.vue'),
    meta: {
      title: 'SQL编辑器',
    },
  },
  {
    name: 'LossyDDLChecker',
    path: '/ddl-check',
    component: () => import('@/views/ddl-precheck/index.vue'),
    meta: {
      title: '有损DDL检查',
    },
  },
  {
    name: '404',
    path: '/404',
    component: () => import('@/views/error-page/404.vue'),
    meta: {
      title: '页面飞走了',
      layout: 'empty',
    },
  },

  {
    name: '403',
    path: '/403',
    component: () => import('@/views/error-page/403.vue'),
    meta: {
      title: '没有权限',
      layout: 'empty',
    },
  },
]
