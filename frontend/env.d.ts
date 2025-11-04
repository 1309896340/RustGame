/// <reference types="vite/client" />
declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  // 定义 Vue SFC 的类型，使其对 TypeScript 可识别
  const component: DefineComponent<{}, {}, any>
  export default component
}