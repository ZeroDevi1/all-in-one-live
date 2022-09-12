<template>
  <el-row>
    <el-col :span="12">
      <el-input v-model="input" placeholder="请输入虎牙直播间号"/>
    </el-col>
    <el-col :span="6">
      <el-button type="primary" @click="getHuyaUrl">获取直播URL</el-button>
    </el-col>
    <el-col :span="6">
      <el-button type="primary" @click="copyHuyaUrl">复制直播URL</el-button>
    </el-col>
  </el-row>
  <el-row>
    <el-col :span="24">
      <video id='live-video' class="live-video"
             ref="videoElement"
             controls autoplay></video>
    </el-col>
  </el-row>

</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {ref, onMounted} from "vue";
import FlvExtend from 'flv-extend'


const input = ref("");
const videoUrl = ref('')

const getHuyaUrl = () => {
  invoke('get_huya_url', {roomId: input.value}).then((res: any) => {
    console.log(res)
    videoUrl.value = res
    initPlayer(res)

  }).catch((err: any) => {
    console.log(err)
  })
}

const handleCopy = (e: ClipboardEvent) => {
  // clipboardData 可能是 null
  e.clipboardData && e.clipboardData.setData('text/plain', videoUrl.value);
  e.preventDefault();
  // removeEventListener 要传入第二个参数
  document.removeEventListener('copy', handleCopy);
};

const copyHuyaUrl = () => {
  // 复制 VideoUrl 的数据到剪切板
  document.addEventListener('copy', handleCopy);
  document.execCommand('copy');

}

let player
let flv: FlvExtend

const buildPlayer = (url: string) => {
  const videoElement = document.getElementById('live-video') as HTMLVideoElement
  // 配置需要的功能
  flv = new FlvExtend({
    element: videoElement, // *必传
    frameTracking: true, // 开启追帧设置
    updateOnStart: true, // 点击播放后更新视频
    updateOnFocus: false, // 获得焦点后更新视频
    reconnect: true, // 开启断流重连
    reconnectInterval: 0 // 断流重连间隔
  })

}

const initPlayer = (url: string) => {
  player = flv.init({
        type: "flv", //=> 媒体类型 flv 或 mp4
        isLive: true, //=> 是否为直播流
        hasAudio: true, //=> 是否开启声音
        url: url, //=> 视频流地址
      }
      , {
        enableStashBuffer: true, // 如果您需要实时（最小延迟）来进行实时流播放，则设置为false
        autoCleanupSourceBuffer: true, // 对SourceBuffer进行自动清理
        stashInitialSize: 128 // 减少首帧显示等待时长
      }
  );
  player.play();
}


onMounted(() => {
  buildPlayer(videoUrl.value)
})


</script>

<style scoped>
.live-video {
  width: 90%;
  margin: 20px;
}
</style>