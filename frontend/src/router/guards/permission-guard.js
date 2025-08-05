const WHITE_LIST = ['/login', '/404']
export function createPermissionGuard(router) {
  router.beforeEach((to) => {
    if (WHITE_LIST.includes(to.path))
      return true
    return true
  })
}