<template>
  <h1 class="text-3xl">登录页面</h1>
  <div class="flex flex-col items-center">
    <div class="bg-gray-400">
      <table class="table-auto">
        <tbody>
          <tr>
            <td class="p-2">
              <span>用户名:</span>
            </td>
            <td class="p-2">
              <input type="text" :v-model="username" :value="username" class="bg-white" />
            </td>
          </tr>
          <tr>
            <td class="p-2">
              <span>密码:</span>
            </td>
            <td class="p-2">
              <input type="password" v-model="password" class="bg-white" />
            </td>
            <td></td>
          </tr>
          <tr>
            <td></td>
            <td>
            </td>
            <td>
              <button class="bg-blue-600 hover:bg-blue-400 m-2 p-1" @click="login">登录</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
const username = ref('windwhisper')
const password = ref('123456')

function login(){
    // 向着后端请求
}
</script>
