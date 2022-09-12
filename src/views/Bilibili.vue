<template>
  <el-row>
    <el-col :span="12">
      <el-input v-model="input" placeholder="请输入哔哩哔哩直播间号"/>
    </el-col>
    <el-col :span="6">
      <el-button type="primary" @click="getBilibiliUrl">获取直播URL</el-button>
    </el-col>
    <el-col :span="6">
      <el-button type="primary" @click="copyHuyaUrl">复制直播URL</el-button>
    </el-col>
  </el-row>
  <el-row>
    <el-col :span="24">
        <vue3videoPlay
            class="bilibili-video"
            v-bind="options"
        />
    </el-col>
  </el-row>

</template>

<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {ref, onMounted, reactive} from "vue";
import 'vue3-video-play/dist/style.css'
import vue3videoPlay from 'vue3-video-play' // 引入组件


const input = ref("");
const videoUrl = ref('')
const options = reactive({
  src: "", //视频源
  type: 'm3u8', //视频类型
  width: '90%', //播放器高度
  // height: '450px', //播放器高度
  color: "#409eff", //主题色
  title: 'Bilibili', //视频名称
  muted: false, //静音
  webFullScreen: false,
  speedRate: ["0.75", "1.0", "1.25", "1.5", "2.0"], //播放倍速
  autoPlay: true, //自动播放
  loop: false, //循环播放
  mirror: false, //镜像画面
  ligthOff: false,  //关灯模式
  volume: 1.0, //默认音量大小
  control: true, //是否显示控制
  controlBtns:['audioTrack', 'quality', 'speedRate', 'volume', 'setting', 'pip', 'pageFullScreen', 'fullScreen'] //显示所有按钮,
})

const getBilibiliUrl = () => {
  invoke('get_bilibili_url', {roomId: input.value}).then((res: any) => {
    console.log(res)
    videoUrl.value = res
    options.src = res
    // initPlayer(res)

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


onMounted(() => {

})


</script>

<style scoped>
.bilibili-video {
  margin: 20px;
  padding: 20px;
}
</style>