<script setup lang="ts">
import {h, onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

const props = defineProps({
  isSelf:Boolean,
  deviceInfo: {
    type: Object,
    required: true
  }
})

const isComputer = ref(false)
const isButtonVisible = ref(false)
onMounted(async () =>{
  if(props.deviceInfo.device_type!="Phone"){
    isComputer.value = true
  }
})
function remove(addr:string){
  console.log(`移除${addr}`)
  invoke('remove_request', {addr:addr}).then()
  ElNotification({
    title: '',
    message: h('p', { style: 'height: auto' }, [
      h(
          'a',
          {
            style: 'margin-top:5px;color: black; font-size: 18px; font-weight: bold;',
          },
          `请求移除${props.deviceInfo.device_name}`
      )
    ]),
    type: `info`,
    customClass:`custom-notification-info`
  })
}
function hideButton(){
  isButtonVisible.value = false
}
function show(){
  isButtonVisible.value = true
}
</script>

<template>
  <div class="container" @click="hideButton" v-bind:class="{ animate: isButtonVisible }">
  <svg v-if="!isComputer" t="1707744541578" class="icon phone" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="7783" width="24" height="24"><path d="M416 832h192v-64h-192v64z" fill="#000000" fill-opacity=".9" p-id="7784"></path><path d="M224 160a64 64 0 0 1 64-64h448a64 64 0 0 1 64 64V896a64 64 0 0 1-64 64h-448a64 64 0 0 1-64-64V160z m64 0V896h448V160h-448z" fill="#000000" fill-opacity=".9" p-id="7785"></path></svg>
  <svg v-if="isComputer" t="1707744425448" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="5072" width="24" height="24"><path d="M853.333333 146.285714a73.142857 73.142857 0 0 1 73.142857 73.142857v463.238096a73.142857 73.142857 0 0 1-73.142857 73.142857H170.666667a73.142857 73.142857 0 0 1-73.142857-73.142857V219.428571a73.142857 73.142857 0 0 1 73.142857-73.142857h682.666666z m0 73.142857H170.666667v463.238096h682.666666V219.428571z m-170.666666 195.047619v73.142858H341.333333v-73.142858h341.333334zM341.333333 804.571429h341.333334v73.142857H341.333333z" p-id="5073"></path></svg>
<!--  <label class="name" v-bind:class="{ isSelf: isSelf }" >{{deviceInfo.device_name}}</label>-->
    <el-tooltip
        class="box-item"
        effect="light"
        :content= props.deviceInfo.device_name
        placement="bottom"
    >
    <el-text class="name" v-bind:class="{ isSelf: isSelf }"  truncated>{{deviceInfo.device_name}}</el-text>
    </el-tooltip>
<!--  <label class="name" v-bind:class="{ isSelf: isSelf }" >{{deviceInfo.device_name}}</label>-->
  <svg t="1707745507392" class="icon spot" @click.stop="show()" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="12062" width="24" height="24"><path d="M510.653 931.287z m0 92.713a92.708 92.708 0 1 0 0-185.426 92.713 92.713 0 0 0 0 185.426M512 512z m0 92.713a92.718 92.718 0 1 0 0-185.431 92.718 92.718 0 0 0 0 185.431m1.347-512z m0 92.713c51.205 0 92.713-41.508 92.713-92.713C606.06 41.508 564.552 0 513.347 0s-92.719 41.508-92.719 92.713c0 51.205 41.513 92.713 92.719 92.713" p-id="12063"></path></svg>
  <button v-show="isButtonVisible" class="red-button" @click.stop="remove(deviceInfo.socket_addr)">解除</button>
  </div>
</template>
<style>
.custom-notification-info {
  background-color: rgba(187, 179, 119, 0.5)!important;
  width: 300px !important;
}
</style>
<style scoped lang="scss">
.icon{
  height: 40px;
  width: 40px;
}
.phone{
  height: 35px;
}
.phone path{
  stroke: black;
  stroke-width:18;
}
.spot{
  height: 20px;
  fill: gray;
}
.spot:hover{
  cursor: pointer;
}
.container {
  display: flex;
  flex-direction: row;
  justify-content: flex-start;
  align-items: center;
  border: 1px red;
  /* 初始状态，没有平移 */
  transform: translateX(0);
  margin-left: 60px;
  /* 定义过渡效果 */
  transition: transform 0.3s ease-in-out;
}

.container > * {
  margin-right: 10px; /* 为每个组件之间添加一些间距 */
}
.name{
  padding-left: 20px;
  font-size: 20px;
  width: 170px;
}
.isSelf{
  color: deepskyblue;
}
.container.animate {
  transform: translateX(-30px);
}


.red-button {
  position: absolute;
  right: 0; /* 初始时放在视线外 */
  top: 0;
  background-color: #cc5a5a;
  color: white;
  border: none;
  padding: 10px;
  cursor: pointer;
  transition: left 0.5s ease; /* 添加过渡效果，持续时间为0.5秒 */
}
</style>