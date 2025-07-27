<template>
  <n-form-item :label="label">
    <slot />
  </n-form-item>
</template>

<script setup lang="ts">
defineProps<{ label: string }>()
</script>
