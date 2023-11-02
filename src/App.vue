<script setup lang="ts">
// import PaperList from './components/PaperList.vue'
// import Menu from './components/Menu.vue'
import {onMounted, ref} from "vue";
import {appWindow, PhysicalSize} from "@tauri-apps/api/window";
const now_window_size = ref<PhysicalSize>()
onMounted(async ()=> {
  // await appWindow.setAlwaysOnTop(true)
  window.addEventListener("contextmenu",  (e) => {e.preventDefault()},false)
  now_window_size.value = await appWindow.innerSize()
  await appWindow.onResized(({payload: size}) => {
    now_window_size.value = size
  })
  // await appWindow.onResized(({payload: size})=>{ //{payload: size}是一个对象。在这个例子中，payload是对象的键，size是对象的值。这种语法是在TypeScript中使用解构分配来创建一个对象。
  //   // console.log(`window resize ${size.width}`)
  // })
// })
})


</script>

<template>
<!--  <div id="bdiv" v-if="now_window_size" :style=" {width:(now_window_size.width * 0.95) + 'px',height:(now_window_size.height * 0.85) + 'px'}">-->
  <div id="bdiv" v-if="now_window_size" :style=" {width:(now_window_size.width-25)+ 'px',height:(now_window_size.height-15)  + 'px'}">
<!--    <h1>这是主界面</h1>-->
    <router-link to="/menu">menu</router-link><br>
    <router-link to="/paperList">paper</router-link><br>
    <router-link to="/markdown1">md1</router-link><br>
    <router-link to="/markdown2" class="text">md2</router-link><br>
    <router-link to="/edit" >edit</router-link><br>
    <keep-alive id="route" >
      <router-view></router-view>
    </keep-alive>
  </div>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
#bdiv{
  border-color:red;
  border-width: 1px;
border-style: solid;
}
#route{
  border-color:blue;
  border-width: 1px;
  border-style: solid;
  height: 100%;
  width: 100%;
}
</style>
