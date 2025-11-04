<template>
  <h1 class="text-3xl">注册</h1>
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
            <td class="p-2">
              <span>重复密码:</span>
            </td>
            <td class="p-2">
              <input type="password" v-model="repassword" class="bg-white" />
            </td>
            <td></td>
          </tr>
          <tr>
            <td></td>
            <td>
              <span v-if="show_verification" class="text-sm text-red-600">{{ verify_password_consistency }}</span>
            </td>
            <td>
              <button class="bg-blue-600 hover:bg-blue-400 m-2 p-1" @click="register">注册</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref, computed, watch } from 'vue'
const username = ref('windwhisper')
const password = ref('123456')
const repassword = ref('123456')

const show_verification = ref(false)
watch(repassword, (newRepassword) => {
  if (newRepassword !== '') {
    show_verification.value = true
  }
})
const verify_password_consistency = computed(() => {
  if(password.value === repassword.value){
    return ""
  }else{
    return "输入密码不一致"
  }
})

function register(){
    // 向着后端请求
}
</script>
