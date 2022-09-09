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
             controls autoplay muted></video>
    </el-col>
  </el-row>
</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {ref, reactive, onMounted} from "vue";
import FlvExtend from 'flv-extend'

const input = ref("lck");
const videoUrl = ref('https://hw.flv.huya.com/src/78941969-2559461636-10992804021986656256-3233009480-10057-A-0-1-imgplus.flv?wsSecret=f8acc2e3ba9010786e76d3f44a0cd6c0&wsTime=631c5f4e&txyp=o%3Anc4%3B&fs=bgct&sphdcdn=al_7-tx_3-js_3-ws_7-bd_2-hw_2&sphdDC=huya&sphd=264_*-265_*&exsphd=264_500,264,264_4000,&t=103&seqid=16627173900000000')


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
  // const videoElement = ref<HTMLVideoElement>()
  // 配置需要的功能
  flv = new FlvExtend({
    element: videoElement, // *必传
    frameTracking: false, // 开启追帧设置
    updateOnStart: true, // 点击播放后更新视频
    updateOnFocus: true, // 获得焦点后更新视频
    reconnect: true, // 开启断流重连
    reconnectInterval: 0 // 断流重连间隔
  })
  player = flv.init({
        type: "flv", //=> 媒体类型 flv 或 mp4
        isLive: true, //=> 是否为直播流
        hasAudio: true, //=> 是否开启声音
        url: url, //=> 视频流地址
      }
      , {
        enableStashBuffer: false, // 如果您需要实时（最小延迟）来进行实时流播放，则设置为false
        autoCleanupSourceBuffer: true, // 对SourceBuffer进行自动清理
        stashInitialSize: 128 // 减少首帧显示等待时长

      }
  );
  player.play();


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
}
</style>