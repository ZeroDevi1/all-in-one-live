<template>
  <el-table :data="liveInfoList" style="width: 100%">
    <el-table-column prop="id" label="ID" width="180"/>
    <el-table-column prop="site_name" label="网站" width="180"/>
    <el-table-column prop="room_id" label="房间号" width="180"/>
    <el-table-column fixed="right" label="操作" width="240">
      <template #default="scope">
        <el-button link type="primary" size="small" @click="getRealUrl(scope.row)"
        >获取直链
        </el-button
        >
        <el-button link type="primary" size="small" @click="toVlc(scope.row)" :disabled="scope.row.direct_url ==null"
        >跳转VLC
        </el-button
        >
        <el-button link type="primary" size="small">删除</el-button>
        <!--        <el-button link type="primary" size="small" @click="toVlc">VLC</el-button>-->
      </template>
    </el-table-column>
  </el-table>
</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {ref, onMounted} from "vue";
import {copyText} from "../libs/copy";
import {ElMessage} from "element-plus";

interface LiveInfo {
  id: number,
  name: string,
  site_name: string,
  site_url: string,
  room_id: string,
  status: string,
  create_time: string,
  direct_url: string,
}

// const toVlc = (url:string) =>{
//   // 在当前页面跳转到 vlc://${url}
//   window.location.href = `vlc://${url}`
// }
const toVlc = (row: LiveInfo) => {
  // 在当前页面跳转到 vlc://${url}
  window.location.href = `vlc://${row.direct_url}`
}
const getRealUrl = (liveInfo: LiveInfo) => {
  console.log(liveInfo.room_id, liveInfo.site_name)
  switch (liveInfo.site_name) {
      // 虎牙直播
    case '虎牙直播':
      invoke('get_huya_url', {roomId: liveInfo.room_id}).then((res: any) => {
        console.log(res)
        copyText(res)
        liveInfo.direct_url = res
        // 提示复制成功
        ElMessage.success('复制成功')
      })
      break;
      // 哔哩哔哩
    case '哔哩哔哩':
      invoke('get_bilibili_url', {roomId: liveInfo.room_id}).then((res: any) => {
        console.log(res)
        copyText(res)
        liveInfo.direct_url = res
        ElMessage.success('复制成功')
      })
      break;
      // 斗鱼直播
  }
}
const liveInfoList = ref<LiveInfo[]>([])

const listLiveInfo = () => {
  invoke('list_live_info').then((res: any) => {
    console.log(res)
    liveInfoList.value = res as LiveInfo[]
  })
}


onMounted(() => {
  listLiveInfo()
})


</script>

<style scoped>

</style>