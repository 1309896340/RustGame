<template>
  <nav class="bg-gray-300">
    <span class="m-2 text-gray-600 hover:text-blue-600 hover:underline" v-for="item in navBar">
        <RouterLink :to="item.path">
            {{ item.name }}
        </RouterLink>
    </span>
  </nav>

  <div>
    <RouterView />
  </div>
</template>

<script lang="ts" setup>
import { RouterView, RouterLink } from 'vue-router'
import { ref } from 'vue'

const navBar = [
  { name: '注册', path: '/register' },
  { name: '登录', path: '/login' },
]
</script>
