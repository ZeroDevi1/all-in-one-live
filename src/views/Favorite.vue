<template>
  <div class="app-container">
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
            <el-button link type="primary" size="small" @click="getRealUrl(scope.row)"
            >获取直链
            </el-button
            >
            <el-button link type="primary" size="small" @click="toVlc(scope.row)"
                       :disabled="scope.row.direct_url ==null"
            >跳转VLC
            </el-button
            >
            <el-button v-if="scope.row.site_name === '哔哩哔哩'" link type="primary" size="small" @click="selectQuality(scope.row)"
            >选择清晰度
            </el-button
            >
            <el-button link type="primary" size="small" @click="delById(scope.row)">删除</el-button>
            <!--        <el-button link type="primary" size="small" @click="toVlc">VLC</el-button>-->
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
          </el-select>
        </el-form-item>
      </el-form>
      <template #footer>
      <span class="dialog-footer">
        <el-button @click="cancelForm">取消</el-button>
        <el-button type="primary" @click="submitForm"
        >确认</el-button
        >
      </span>
      </template>
    </el-dialog>
    <el-dialog
        v-model="dialogVisible"
        title="选择清晰度"
        width="30%"
        :before-close="handleClose"
    >
      <el-table :data="qualityList" style="width: 100%">
        <el-table-column prop="desc" label="desc" width="180"/>
        <el-table-column fixed="right" label="操作" width="240">
          <template #default="scope">
            <el-button link type="primary" size="small" @click="copyUrl(scope.row)"
            >复制直链
            </el-button>
            <!--          <el-button link type="primary" size="small" @click="toVlc(scope.row)"-->
            <!--                     :disabled="scope.row.direct_url ==null"-->
            <!--          >跳转VLC-->
            <!--          </el-button-->
            <!--          >-->
            <!--          <el-button link type="primary" size="small" @click="delById(scope.row)">删除</el-button>-->
            <!--          &lt;!&ndash;        <el-button link type="primary" size="small" @click="toVlc">VLC</el-button>&ndash;&gt;-->
          </template>
        </el-table-column>
      </el-table>
      <!--    <template #footer>-->
      <!--      <span class="dialog-footer">-->
      <!--        <el-button @click="dialogVisible = false">Cancel</el-button>-->
      <!--        <el-button type="primary" @click="dialogVisible = false"-->
      <!--        >Confirm</el-button-->
      <!--        >-->
      <!--      </span>-->
      <!--    </template>-->
    </el-dialog>
  </div>

</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {ref, onMounted} from "vue";
import {copyText} from "../libs/copy";
import {ElMessage, ElMessageBox} from "element-plus";
import {Quality} from "../libs/type";

const dialogFormVisible = ref(false)
const title = ref('')
const qualityList = ref<Quality[]>()
const dialogVisible = ref(false)

const handleClose = (done: () => void) => {
  ElMessageBox.confirm('Are you sure to close this dialog?')
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
// 获取直播直链
const getRealUrl = (liveInfo: LiveInfo) => {
  switch (liveInfo.site_name) {
      // 虎牙直播
    case '虎牙直播':
      invoke('get_huya_url', {roomId: liveInfo.room_id}).then((res: any) => {
        copyText(res)
        liveInfo.direct_url = res
        // 提示复制成功
        ElMessage.success('复制成功')
      })
      break;
      // 哔哩哔哩
    case '哔哩哔哩':
      invoke('get_bilibili_url', {roomId: liveInfo.room_id}).then((res: any) => {
        copyText(res)
        liveInfo.direct_url = res
        ElMessage.success('复制成功')
      })
      break;
      // 斗鱼直播
  }
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
const copyUrl = (quality: Quality) => {
  copyText(quality.url)
  dialogVisible.value = false
}

// 选择清晰度
const selectQuality = (liveInfo: LiveInfo) => {
  invoke('get_bilibili_urls_with_quality', {roomId: liveInfo.room_id}).then((res: any) => {
    qualityList.value = res
    dialogVisible.value = true
  })
}

onMounted(() => {
  listLiveInfo()
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