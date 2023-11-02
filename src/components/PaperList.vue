<template>
<!--  <div style="border-color:green;border-width: 1px;border-style: solid;" :style="{width:window_width*0.9+'px'}" >-->
  <div style="border-color:green;border-width: 1px;border-style: solid;width: 80%;height: 80%"  >
<!--  <div style="height:800px;" >-->
<!--    <el-auto-resizer>-->
<!--      <template #default="{ height, width }">-->
<!--        <el-table-v2-->
<!--            :columns="columns"-->
<!--            :data="tableData"-->
<!--            :width="width"-->
<!--            :height="height"-->
<!--            fixed-->
<!--        />-->
<!--      </template>-->
<!--    </el-auto-resizer>-->
<!--  </div>-->
<!--  <el-table :data="tableData" stripe style="width: 100%" max-height="600rem">-->
<!--  <el-table :data="tableData" stripe style="width: 100%;height: 70%" :max-height="window_width*0.7">-->
  <el-table :data="tableData" stripe style="width: 100%;height: 90%">
    <el-table-column prop="name" label="Name" width="600" >
    <template #default="scope">
      <div style="display:flex;flex-direction: column">
        <el-link type="default" @click="openPaper(scope.row.path,$event)">{{ scope.row.name }}</el-link>
        <el-tag>{{ scope.row.abstracts }}</el-tag>
      </div>
    </template>
    </el-table-column>
  </el-table>
<!--  <h1>{{ msg }}</h1>-->
<!--  <h1>{{ count }}</h1>-->
<!--    <label>{{ tableData }}</label>-->
  <div style="width: 90%">
    <!-- 由于SVG图标默认不携带任何属性 -->
    <!-- 你需要直接提供它们 -->
    <el-row class="mb-4">
<!--      <el-button round>Round</el-button>-->
<!--      <el-button type="primary" round>Primary</el-button>-->
<!--      <el-button type="success" round>Success</el-button>-->
<!--      <el-button type="info" round>Info</el-button>-->
<!--      <el-button type="warning" round>Warning</el-button>-->
<!--      <el-button type="danger" @click.left = "newWindow()" round>Danger</el-button>-->
<!--    </el-row>-->
<!--    <Edit style="width: 1em; height: 1em; margin-right: 8px" />-->
<!--    <Share style="width: 1em; height: 1em; margin-right: 8px" />-->
<!--    <Delete style="width: 1em; height: 1em; margin-right: 8px" />-->
<!--    <Search style="width: 1em; height: 1em; margin-right: 8px" />-->
<!--    <el-row>-->
      <el-button :icon="FolderOpened" @click="openFolder" style="border:none"/>
      <el-button :icon="DocumentAdd" @click="selectPdf" style="border:none"/>
<!--      <el-button type="primary" :icon="Edit" circle />-->
    </el-row>
  </div>
  </div>
</template>

<script setup lang="tsx" >
import {onMounted, ref} from 'vue'
import {Check, Delete, DocumentAdd, Edit, FolderOpened, Message, Search, Share, Star} from "@element-plus/icons-vue";
import {
  ElButton,
} from 'element-plus'
import { open } from "@tauri-apps/api/dialog";
import { invoke } from '@tauri-apps/api'
import {appWindow, WebviewWindow} from "@tauri-apps/api/window";
defineProps<{ msg: string }>()
type paper = {
  id:string,
  name:string,
  path:string,
  abstracts:string,
  content:string,
}
const tableData = ref<paper[]>([]);
const window_width = ref<number>(0)
onMounted(async ()=>{
  const size = await appWindow.innerSize()
  window_width.value =size.width
  await appWindow.onResized(({payload: size}) => {
    window_width.value = size.width
    // console.log(window_width.value*0.7)
  })
  invoke<paper[]>('query_all_pdf')
      .then((response)=>{
        console.log(response)
        tableData.value.unshift(...response)
      })
})

function newWindow(){
  const webview: WebviewWindow = new WebviewWindow('theUniqueLabel', {
    // url: "http://localhost:5173/#/menu"
    url: "https://xinghuo.xfyun.cn/desk"
  });
}

const count = ref(0)
// const papers = ref([])
async function openFolder(){
  let folders: string | string[] | null = await open({
    title:"选择文件夹",
    multiple:true,
    directory:true
  })
  console.log(`选择文件夹${folders}`)
  if (folders!=null){
    // invoke<paper[]>('get_all_pdf', {dirs: folders})
    invoke<paper[]>('import_all_pdf', {dirs: folders})
        .then((response)=>{
          console.log(response)
          tableData.value = response
          // response.forEach((paper)=>{
          //   // tableData.value.unshift({name:paper.name,abstracts:paper.abstracts})
          //   tableData.value.unshift(paper)
          // })
        })
  }
}
async function selectPdf(){
  let files = await open({
    title:"选择pdf",
    multiple:true,
    filters:[{
      name:"PDF",
      extensions:["pdf"]
    }]
  })
  console.log(`选择pdf${files}`)
  if (files != null) {
    invoke<paper>('greet', {name: files[0]})
        // `invoke` 返回的是一个 Promise
        .then((response) => {
          console.log(response)
          count.value += 1
          // tableData.value.push({name:response.url,age:response.id,gender: "male",tel: response.content})
        })
  }
}
// const columns = [
//   { key: 'name', dataKey: 'name', title: 'Name', width: 600 ,align:'center',
//     cellRenderer: ({ cellData: name }) => (<el-tag>nam</el-tag>),
//   },
//   // { key: 'tel', dataKey: 'tel', title: 'Tel', width: 100 }
// ]
// const tableData: Ref<UnwrapRef<{name:string}>>= ref([
// // const tableData= ref([
//   // { name: 'ni', age: 12, gender: '16', tel: '19' },
//   // ...
// ])
// const tableData = ref<{ name: string;abstracts:string }[]>([]);
// const tableData = ref<paper[]>([]);
function openPaper(path:string,event:MouseEvent){
  if (event.button==0){
    invoke<boolean>('open_paper',{path: path})
        .then((response) =>{
          if (response){
            console.log("success")
          }else {
            console.log("false")
          }
        })
  }
}
</script>


<style scoped>
.read-the-docs {
  color: #888;
}
</style>



