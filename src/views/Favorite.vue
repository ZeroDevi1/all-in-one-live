<template>
  <div class="app-container">
    <div id="mui-player"></div>
    <el-row :gutter="20">
      <!--  新增按钮-->
      <el-button type="primary" @click="openAddForm">新增</el-button>
    </el-row>
    <el-row :gutter="20">
      <el-table :data="liveInfoList" style="width: 100%">
        <el-table-column prop="id" label="ID" width="180"/>
        <el-table-column prop="site_name" label="网站" width="180"/>
        <el-table-column prop="room_id" label="房间号" width="180"/>
        <el-table-column fixed="right" label="操作" width="290">
          <template #default="scope">
            <el-button link type="primary" size="small" @click="getRealUrl(scope.row)">获取直链
            </el-button>
            <el-button link type="primary" size="small" @click="toVlc(scope.row)"
                       :disabled="scope.row.direct_url ==null">跳转VLC
            </el-button>
            <el-button link type="primary" size="small" @click="selectQuality(scope.row)">选择清晰度
            </el-button>
            <el-button link type="primary" size="small" @click="delById(scope.row)">删除</el-button>
          </template>
        </el-table-column>
      </el-table>
    </el-row>

    <el-dialog v-model="dialogFormVisible" :title="title" @close="cancelForm">
      <el-form :model="model">
        <el-form-item label="房间号">
          <el-input v-model="model.room_id"/>
        </el-form-item>
        <el-form-item label="直播网站">
          <el-select v-model="model.site_name" placeholder="请选择直播网站">
            <el-option label="虎牙直播" value="虎牙直播"/>
            <el-option label="哔哩哔哩" value="哔哩哔哩"/>
            <el-option label="斗鱼直播" value="斗鱼直播"/>
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
        <span class="dialog-footer">
          <el-button @click="cancelForm">取消</el-button>
          <el-button type="primary" @click="submitForm">确认</el-button>
        </span>
      </template>
    </el-dialog>
    <el-dialog v-model="dialogVisible" title="选择清晰度" width="36%" :before-close="handleClose">
      <el-table :data="qualityList" style="width: 100%">
        <el-table-column prop="desc" label="desc" width="180"/>
        <el-table-column fixed="right" label="操作" width="240">
          <template #default="scope">
            <el-button link type="primary" size="small" @click="copyUrl(scope.row)">复制直链
            </el-button>
            <el-button link type="primary" size="small" @click="toVlcQuality(scope.row)"
                       :disabled="scope.row.url ==null">跳转VLC
            </el-button>

          </template>
        </el-table-column>
      </el-table>

    </el-dialog>
  </div>

</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {ref, onMounted} from "vue";
import {ElMessage, ElMessageBox} from "element-plus";
import {Quality} from "../libs/type";


import 'mui-player/dist/mui-player.min.css'
// @ts-ignore
import MuiPlayer from 'mui-player'
// 使用模块管理器引入插件
// @ts-ignore
import MuiPlayerDesktopPlugin from 'mui-player-desktop-plugin'
import Hls from "hls.js";
import Flv from 'flv.js'

// clipboard
import {writeText, readText} from '@tauri-apps/api/clipboard';


// 初始化 MuiPlayer 插件，MuiPlayer 方法传递一个对象，该对象包括所有插件的配置
let mp: MuiPlayer;

const buildPlayer = (url: string) => {
  // 如果 url 是 m3u8 格式的直播流，可以使用 hls.js 来播放
  if (url.indexOf('.m3u8') > -1) {
    mp = new MuiPlayer({
      container: '#mui-player',
      title: '标题',
      src: url,
      parse: {
        type: 'hls',
        loader: Hls,
        config: { // hls config to：https://github.com/video-dev/hls.js/blob/HEAD/docs/API.md#fine-tuning
          debug: false,
        },
      },
      plugins: [
        new MuiPlayerDesktopPlugin({
          customSetting: [], // 设置组配置
          contextmenu: [], // 右键菜单组配置
          thumbnails: [],  // 缩略图配置
        })
      ]
    })
  } else {
    mp = new MuiPlayer({
      container: '#mui-player',
      title: '标题',
      src: url,
      parse: {
        type: 'flv',
        loader: Flv, // flv config to：https://github.com/Bilibili/flv.js/blob/HEAD/docs/api.md#flvjscreateplayer
        config: {
          cors: true
        },
      },
      plugins: [
        new MuiPlayerDesktopPlugin({
          customSetting: [], // 设置组配置
          contextmenu: [], // 右键菜单组配置
          thumbnails: [],  // 缩略图配置
        })
      ]
    })
  }

}

const dialogFormVisible = ref(false)
const title = ref('')
const qualityList = ref<Quality[]>()
const dialogVisible = ref(false)

const handleClose = (done: () => void) => {
  ElMessageBox.confirm('是否关闭选择框?')
      .then(() => {
        done()
        // dialogVisible.value = false
      })
      .catch(() => {
        // catch error
      })
}

interface LiveInfo {
  id?: number,
  name: string,
  site_name: string,
  site_url: string,
  room_id: string,
  status: string,
  create_time: string,
  direct_url: string,
}

const model = ref<LiveInfo>({
  name: '',
  site_name: '',
  site_url: '',
  room_id: '',
  status: '',
  create_time: '',
  direct_url: '',
})

const toVlc = (row: LiveInfo) => {
  // 在当前页面跳转到 vlc://${url}
  window.location.href = `vlc://${row.direct_url}`
}

const toVlcQuality = (row: Quality) => {
  // 在当前页面跳转到 vlc://${url}
  window.location.href = `vlc://${row.url}`
}
// 获取直播直链
const getRealUrl = (liveInfo: LiveInfo) => {
  switch (liveInfo.site_name) {
      // 虎牙直播
    case '虎牙直播':
      invoke('get_huya_url', {roomId: liveInfo.room_id}).then(async (res: any) => {
        await writeText(res)
        liveInfo.direct_url = res
        // 提示复制成功
        ElMessage.success('复制成功')
      })
      break;
      // 哔哩哔哩
    case '哔哩哔哩':
      invoke('get_bilibili_url', {roomId: liveInfo.room_id}).then(async (res: any) => {
        await writeText(res)
        liveInfo.direct_url = res
        ElMessage.success('复制成功')
      })
      break;
      // 斗鱼直播
    case '斗鱼直播':
      invoke('get_douyu_url', {roomId: liveInfo.room_id}).then(async (res: any) => {
        await writeText(res)
        liveInfo.direct_url = res
        ElMessage.success('复制成功')
      })
      break;
  }
  buildPlayer(liveInfo.direct_url)
}
// 存储的直播信息
const liveInfoList = ref<LiveInfo[]>([])

// 刷新存储的直播信息
const listLiveInfo = () => {
  invoke('list_live_info').then((res: any) => {
    liveInfoList.value = res as LiveInfo[]
  })
}

// 删除直播间
const delById = (liveInfo: LiveInfo) => {
  invoke('del_live_info_by_id', {id: liveInfo.id}).then((res: any) => {
    listLiveInfo()
  })
}

// 新增直播间
const openAddForm = () => {
  dialogFormVisible.value = true
  title.value = '新增'
}

// 保存直播间
const submitForm = () => {
  // 根据直播网站设置 siteUrl
  switch (model.value.site_name) {
    case '虎牙直播':
      model.value.site_url = 'https://www.huya.com/'
      break;
    case '哔哩哔哩':
      model.value.site_url = 'https://live.bilibili.com/'
      break;
    case '斗鱼直播':
      model.value.site_url = 'https://www.douyu.com/'
      break;
  }
  model.value.name = model.value.room_id
  invoke('add_live_info', {"liveInfo": model.value}).then((res: any) => {
    listLiveInfo()
    dialogFormVisible.value = false
    // 清空 model
    model.value = {
      name: '',
      site_name: '',
      site_url: '',
      room_id: '',
      status: '',
      create_time: '',
      direct_url: '',
    }
  })
}

// 取消新增直播间
const cancelForm = () => {
  dialogFormVisible.value = false
  // 清空 model
  model.value = {
    name: '',
    site_name: '',
    site_url: '',
    room_id: '',
    status: '',
    create_time: '',
    direct_url: '',
  }
}

// 复制链接
const copyUrl = async (quality: Quality) => {
  await writeText(quality.url);
  dialogVisible.value = false
}

// 选择清晰度
const selectQuality = (liveInfo: LiveInfo) => {
  switch (liveInfo.site_name) {
    case '哔哩哔哩':
      invoke('get_bilibili_urls_with_quality', {roomId: liveInfo.room_id}).then((res: any) => {
        qualityList.value = res
        dialogVisible.value = true
      })
      break;
    case '虎牙直播':
      invoke('get_huya_urls_with_quality', {roomId: liveInfo.room_id}).then((res: any) => {
        qualityList.value = res
        dialogVisible.value = true
      })
      break;
    case '斗鱼直播':
      invoke('get_douyu_urls_with_quality', {roomId: liveInfo.room_id}).then((res: any) => {
        qualityList.value = res
        dialogVisible.value = true
      })
      break;
  }

}

onMounted(() => {
  listLiveInfo()
  buildPlayer('')
})


</script>

<style scoped>
.el-button--text {
  margin-right: 15px;
}

.el-select {
  width: 300px;
}

.el-input {
  width: 300px;
}

el-row {
  margin: 10px;
}

.dialog-footer button:first-child {
  margin-right: 10px;
}
</style>