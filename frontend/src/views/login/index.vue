<template>
  <div class="wh-full flex-col bg-[url(@/assets/images/login_bg.webp)] bg-cover">
    <div class="m-auto max-w-700 min-w-345 f-c-c rounded-8 auto-bg bg-opacity-20 bg-cover p-12 card-shadow">
      <div class="w-320 flex-col px-20 py-32">
        <h2 class="f-c-c text-24 text-#6a6a6a font-normal">
          <img src="@/assets/images/logo.png" class="mr-12 h-50 cursor-pointer" @click="goLogin" />
          {{ title }}
        </h2>
        <div class="mt-20 flex items-center">
          <n-button class="h-40 flex-1 rounded-5 text-16" type="primary" ghost @click="quickLogin">
            一键体验
          </n-button>
        </div>
      </div>
    </div>
    <TheFooter class="py-12" />
  </div>
</template>

<script setup>
const router = useRouter()
const title = import.meta.env.VITE_TITLE

function goLogin() {
  // 兼容 hash/history 路由，始终跳转到 /login
  if (router.currentRoute.value.path !== '/login') {
    router.push('/login')
  } else {
    window.location.reload()
  }
}

function quickLogin() {
  router.push('/')
}
</script>
