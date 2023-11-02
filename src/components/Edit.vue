<script setup lang="ts">
import '@wangeditor/editor/dist/css/style.css' // 引入 css

import {onBeforeUnmount, ref, shallowRef, onMounted, watch} from 'vue'
import { Editor, Toolbar } from '@wangeditor/editor-for-vue'
import {Boot, createEditor, IEditorConfig, IToolbarConfig} from '@wangeditor/editor'
import formulaModule from '@wangeditor/plugin-formula'
import {IDomEditor} from "@wangeditor/core";
import {invoke, path} from "@tauri-apps/api";
import {readBinaryFile, writeBinaryFile} from "@tauri-apps/api/fs";

Boot.registerModule(formulaModule)

const editorRef = shallowRef<IDomEditor>()

// 内容 HTML
const valueHtml = ref('<p>hello</p>' +
    '<img src="image.jpg" alt="描述图片的文本">')

const  mode = 'default'

// 模拟 ajax 异步获取内容
onMounted(() => {
  setTimeout(() => {
    valueHtml.value = '<p>模拟 Ajax 异步设置内容</p><img src="/edit/img.png" style="width: 100px;height: 200px" alt="描述图片的文本">'
  }, 1500)
})


// 工具栏配置
const toolbarConfig: Partial<IToolbarConfig> = {
  insertKeys: {
    index: 0,
    keys: [
      'insertFormula', // “插入公式”菜单
      'editFormula' // “编辑公式”菜单
    ],
  },

  // 其他...
}
const editorConfig:Partial<IEditorConfig> = {
  placeholder: '请输入内容...' ,
  // 选中公式时的悬浮菜单
  hoverbarKeys:{
    formula:{
      menuKeys: ['editFormula'], // “编辑公式”菜单
    }
  },
  MENU_CONF: {}
}
// editorConfig.customPaste = (editor:IDomEditor,c_event:ClipboardEvent): boolean =>{
//   console.log("粘贴类型是"+c_event.clipboardData.types)
//   return true
// }
// editorConfig.MENU_CONF['uploadImage'] = {
//
// }

// 组件销毁时，也及时销毁编辑器
onBeforeUnmount(() => {
  const editor = editorRef.value
  if (editor == null) return
  editor.destroy()
})

const handleCreated = (editor:IDomEditor) => {
  editorRef.value = editor // 记录 editor 实例，重要！

}
// const testFn = async () => {
//   let imagePath = await invoke('read_clipboard_string') as Array<string>;
//   if (imagePath[0].length==0){
//    console.log("mieyou tupiuan")
//     return
//   }
//   console.log(imagePath)
//   const contents = await readBinaryFile(imagePath[0]);
//   console.log(contents)
//   await writeBinaryFile('F:\\WorkSpace\\Rust\\RustRover\\paper-enhancement\\public\\edit\\avatar1.png', contents);
//   console.log("success")
// };
async function sendArrayBufferToRust(
    path: string,
    arrayBuffer: ArrayBuffer,
    progressCallback?: (progress: number) => void
) {
  const chunkSize = 1024 * 1024 * 10; // 10MB
  const numChunks = Math.ceil(arrayBuffer.byteLength / chunkSize);
  for (let i = 0; i < numChunks; i++) {
    const start = i * chunkSize;
    const end = Math.min(start + chunkSize, arrayBuffer.byteLength);
    const chunk = arrayBuffer.slice(start, end);
    if (!(await invoke("append_chunk_to_file", {path: path, chunk:Array.from(new Uint8Array(chunk))}))) {
      console.log("error");
      return false;
    }
    if (progressCallback) {
      const progress = (i + 1) / numChunks;
      progressCallback(progress);
    }
  }
  return true;
}
async function customPaste(editor: IDomEditor, event: ClipboardEvent) {
  if (event.clipboardData!.types[0] == "Files") {
    console.log("粘贴类型是文件")
    // testFn()
    const file = event.clipboardData!.files[0]
    const content = await file.arrayBuffer()
    console.log(`文件大小是${content.byteLength}`)
    await writeBinaryFile(`F:\\WorkSpace\\Rust\\RustRover\\paper-enhancement\\public\\edit\\avatar1.png`, content);

    // sendArrayBufferToRust("F:\\WorkSpace\\Rust\\RustRover\\paper-enhancement\\public\\edit\\avatar1.png", content)
    // console.log(await path.downloadDir())
    //   await writeBinaryFile(`${await path.downloadDir()}/avatar1.png`, (await content),undefined);
    // await writeBinaryFile(`F:\\WorkSpace\\Rust\\RustRover\\paper-enhancement\\public\\edit\\avatar1.png`, content);
    //   await writeBinaryFile('avatar.png', fileArray, { dir: BaseDirectory.AppData });
    // };
  }
  return true
}

// editorConfig.MENU_CONF['uploadImage'] = {
  // 自定义上传
  // async customUpload(file: File, insertFn: InsertFnType) {  // TS 语法
  //   // async customUpload(file, insertFn) {                   // JS 语法
  //   // file 即选中的文件
  //   // 自己实现上传，并得到图片 url alt href
  //   // 最后插入图片
  //   insertFn(url, alt, href)
  // }
// }

</script>

<template>
  <div style="border: 1px solid #ccc">
    <Toolbar
        style="border-bottom: 1px solid #ccc"
        :editor="editorRef"
        :defaultConfig="toolbarConfig"
        :mode="mode"
    />
    <Editor
        style="height: 500px; overflow-y: hidden;"
        v-model="valueHtml"
        :defaultConfig="editorConfig"
        :mode="mode"
        @onCreated="handleCreated"
        @customPaste="customPaste"
    />
  </div>
</template>

<style scoped>

</style>