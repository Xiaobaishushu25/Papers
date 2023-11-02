<script setup lang="ts">
import { ref } from 'vue';

// const text = ref('');
const text = ref(`
## **依赖的库**

待整理..😅

## **模块化**
1. jsch不支持模块化，但是有简单迁移的替代方法：https://github.com/mwiede/jsch  <br>
2. h2数据库：截止至2023年9月11日18:48:59不支持模块化：https://github.com/h2database/h2database/issues/1415  <br>
3. eawtstub：不支持模块化，但是我fork了一份 https://github.com/Xiaobaishushu25/eawtstub，需要时可以自己模块化一下  <br>
4. trove4j <br>
5. slf4j-log4j12 <br>
6. jna：支持模块化
7. 其他....

## **打赏或者贡献** 🥰

您可以通过下面的方法来贡献和支持该项目：
* 在 GitHub/Gitee 上为项目加注星标、给予反馈
* 提交PR
* 贡献您的想法建议
* 与您的同学朋友同事分享Bshell
* 如果您喜欢Bshell，可以考虑捐赠
`);
</script>

<template>
  <!-- :disabled-menus="[]"把禁用的title放在此数组中
         :include-level="[1,2,3,4,5,6]" 点击目录导航的层级
         @upload-image="handleUploadImage"图片上传
         @change="change"双向绑定效果
      -->
  <v-md-editor v-model="text" height="400px" style="width: 500px"></v-md-editor>
</template>

<style scoped>

</style>