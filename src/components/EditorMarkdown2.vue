<template>
<!--  在使用 'MdEditor' 组件之前确保 'now_window_size' 已经被赋值，使用 Vue 的条件渲染，只有在 'now_window_size' 有值时才渲染 'MdEditor' 组件-->
<!--  <div v-if="now_window_size" class="bdiv">-->
  <div class="bdiv">
<!--    <MdEditor v-model="text" :style="{width: (now_window_size.width * 0.95) + 'px',height:(now_window_size.height * 0.85) + 'px'}" />-->
    <MdEditor v-model="text"  @contextmenu="onContextMenu($event)"  @blur="blurChange($event)" style="width: 100%;height: 80%" />
  </div>
<!--  <MdEditor v-model="text" :style="{width: (now_window_size.width*0.99)+'px'}"/>-->
</template>

<script  lang="ts"  setup>
import { MdEditor } from 'md-editor-v3';
import 'md-editor-v3/lib/style.css';
import { PhysicalSize } from "@tauri-apps/api/window";

import ContextMenu from '@imengyu/vue3-context-menu'

import {h, onMounted, ref, watch} from 'vue';
function blurChange( _:FocusEvent){
  ContextMenu.closeContextMenu
}
//右键菜单
function onContextMenu(e : MouseEvent) {
  //prevent the browser's default menu
  e.preventDefault();
  //show your menu
  ContextMenu.showContextMenu({
    x: e.x,
    y: e.y,
    items: [
      {
        label: "A menu item",
        icon: h('div', {
          style: {
            width: '20px',
            height: '20px',
          }
        },{}),
        onClick: () => {
          alert("You click a menu item");
        }
      },
      {
        label: "A submenu1",
        icon: h('img', {
          // src: 'https://imengyu.top/assets/images/test/icon.png',
          src: '../src/assets/icons/copy24.png',
          style: {
            width: '20px',
            height: '20px',
          }
        }),
        children: [
          { label: "Item1" },
          { label: "Item2" },
          { label: "Item3" },
        ]
      },
      {
        label: "A submenu2",
        children: [
          { label: "Item1" },
          { label: "Item2" },
          { label: "Item3" },
        ]
      },
      {
        label: "A submenu3",
        children: [
          { label: "Item1" },
          { label: "Item2" },
          { label: "Item3" },
        ]
      },
      {
        label: "A submenu4",
        children: [
          { label: "Item1" },
          { label: "Item2" },
          { label: "Item3" },
        ]
      },
      {
        label: "A submenu5",
        children: [
          { label: "Item1" },
          { label: "Item2" },
          { label: "Item3" },
        ]
      },
      {
        label: "A submenu6",
        children: [
          { label: "Item1" },
          { label: "Item2" },
          { label: "Item3" },
        ]
      },
      {
        label: "A submenu7",
        children: [
          { label: "Item1" },
          { label: "Item2" },
          { label: "Item3" },
        ]
      },
      {
        label: "A submenu8",
        children: [
          { label: "Item1" },
          { label: "Item2" },
          { label: "Item3" },
        ]
      },
      {
        label: "A submenu9",
        children: [
          { label: "Item1" },
          { label: "Item2" },
          { label: "Item3" },
        ]
      },
      {
        label: "A submenu10",
        children: [
          { label: "Item1" },
          { label: "Item2" },
          { label: "Item3" },
        ]
      },
      {
        label: "A submenu",
        children: [
          { label: "Item1" },
          { label: "Item2" },
          { label: "Item3" },
        ]
      },
      {
        label: "A submenu",
        children: [
          { label: "Item1" },
          { label: "Item2" },
          { label: "Item3" },
        ]
      },
    ]
  });
}
const now_window_size = ref<PhysicalSize>()
const isContextMenuVisible = ref(false)
const contextMenuX = ref(0)
const contextMenuY = ref(0)
function hideRightMenu(event:MouseEvent){
  isContextMenuVisible.value = false
}
function showContextMenu(event:MouseEvent) {
  // event.preventDefault();
  isContextMenuVisible.value = true;
  // contextMenuX.value = event.screenX;
  // contextMenuY.value = event.screenY;
  // contextMenuX.value = event.clientX;
  // contextMenuY.value = event.clientY;
  contextMenuX.value = event.pageX;
  contextMenuY.value = event.pageY;
  console.log(`右键位置是${contextMenuX.value} ${contextMenuY.value} `)
}
function hideContextMenu(){
  isContextMenuVisible.value = false
}
function handleMenuItemClick(item:string) {
  // 处理右键菜单项的点击事件
  console.log(`Clicked on ${item}`);
  hideContextMenu();
}

// onMounted(async ()=>{
//   now_window_size.value = await appWindow.innerSize()
//   await appWindow.onResized(({payload: size}) => {
//     now_window_size.value = size
//   })
// })
const text = ref(`
## **依赖的库**
待整理..😅
## **模块化**
1. jsch不支持模块化，但是有简单迁移的替代方法：https://github.com/mwiede/jsch  <br>
## **打赏或者贡献** 🥰
您可以通过下面的方法来贡献和支持该项目：
* 在 GitHub/Gitee 上为项目加注星标、给予反馈
`);
watch(text,async (newText,_)=>{
  console.log(`new 是 ${newText}`)
})
function rightMenu(event:MouseEvent){
  console.log("右键菜单")
  showContextMenu(event)
}
</script>
<style>
/*.bdiv{
  border-color:green;
  border-width: 1px;
  border-style: solid;
}*/
.right-click-me {
  width: 200px;
  height: 100px;
  background-color: lightgray;
  text-align: center;
  line-height: 100px;
}

.context-menu {
  /*position: absolute;*/
  position: fixed;
  background-color: white;
  border: 1px solid #ccc;
  box-shadow: 2px 2px 5px rgba(0, 0, 0, 0.2);
  max-width: 200px;
  z-index: 999;
  top: 0;
  left: 0;
}

.context-menu ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.context-menu li {
  padding: 8px;
  cursor: pointer;
}

.context-menu li:hover {
  background-color: #f0f0f0;
}
</style>