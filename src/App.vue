<template>
  <label class="pair-title">配对码</label>
  <div v-loading="loading"
       :element-loading-spinner="svg"
       element-loading-svg-view-box="-10, -10, 50, 50"
       element-loading-background="rgba(246,246,246, 0.8)">
    <div class="verification-container" >
      <input
          v-for="(code, index) in pairCode"
          :key="index"
          :disabled = pairState
          v-model="pairCode[index]"
          @input="handleInput(index, $event)"
          @keydown="handleKeyDown(index, $event)"
          type="number"
          maxlength="1"
          class="verification-input no-spinners"
      />
    </div>
    <div  class="pair-state" >
      <button v-if="!pairState" @click="pairCreateRequest()" class="pair-false">没有配对码，请求创建.</button>
      <button v-if="pairState" disabled class="pair-true">配对成功.</button>
      <svg t="1707836252448" class="icon" @click="copy()" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="5919" width="24" height="24"><path d="M857.373005 65.290005 469.604424 65.290005c-34.211173 0-62.044078 27.832905-62.044078 62.043055l0 10.340509-63.076594 0c-25.993001 0-48.228421 16.346293-57.001225 39.293935L166.626995 176.967504c-34.21015 0-62.043055 27.832905-62.043055 62.043055l0 657.655358c0 34.21015 27.832905 62.043055 62.043055 62.043055l550.115086 0c34.21015 0 62.043055-27.832905 62.043055-62.043055l0-49.634444 78.587869 0c34.21015 0 62.043055-27.832905 62.043055-62.043055L919.41606 127.33306C919.41606 93.122911 891.583155 65.290005 857.373005 65.290005zM344.483752 179.035606l194.402595 0c10.833743 0 19.646456 8.813736 19.646456 19.646456 0 10.833743-8.813736 19.646456-19.646456 19.646456L344.483752 218.328517c-10.833743 0-19.646456-8.813736-19.646456-19.646456C324.836273 187.849342 333.650009 179.035606 344.483752 179.035606zM737.423099 896.665917c0 11.402701-9.278317 20.681018-20.681018 20.681018L166.626995 917.346935c-11.403724 0-20.681018-9.278317-20.681018-20.681018L145.945977 239.010559c0-11.402701 9.277294-20.681018 20.681018-20.681018l120.111588 0c8.197706 24.02723 30.977525 41.362037 57.744145 41.362037l194.402595 0c26.767644 0 49.54644-17.334807 57.744145-41.362037l120.111588 0c11.402701 0 20.681018 9.278317 20.681018 20.681018L737.422076 896.665917zM878.054023 784.988418c0 11.402701-9.278317 20.681018-20.681018 20.681018l-78.587869 0L778.785136 239.010559c0-34.21015-27.832905-62.043055-62.043055-62.043055L595.886549 176.967504c-8.771781-22.947641-31.007201-39.293935-57.001225-39.293935l-89.963964 0L448.921359 127.33306c0-11.403724 9.278317-20.681018 20.683065-20.681018l387.768581 0c11.402701 0 20.681018 9.277294 20.681018 20.681018L878.054023 784.988418z" fill="#272636" p-id="5920"></path><path d="M620.597347 334.252737 260.748652 334.252737c-11.422144 0-20.681018 9.259898-20.681018 20.681018s9.258874 20.681018 20.681018 20.681018l359.849718 0c11.42112 0 20.681018-9.259898 20.681018-20.681018S632.018467 334.252737 620.597347 334.252737z" fill="#272636" p-id="5921"></path><path d="M620.597347 454.201619 260.748652 454.201619c-11.422144 0-20.681018 9.259898-20.681018 20.681018 0 11.42112 9.258874 20.681018 20.681018 20.681018l359.849718 0c11.42112 0 20.681018-9.259898 20.681018-20.681018C641.278365 463.46254 632.018467 454.201619 620.597347 454.201619z" fill="#272636" p-id="5922"></path><path d="M440.673511 574.151525 260.748652 574.151525c-11.422144 0-20.681018 9.259898-20.681018 20.681018 0 11.42112 9.258874 20.681018 20.681018 20.681018l179.924859 0c11.42112 0 20.681018-9.259898 20.681018-20.681018C461.35453 583.411423 452.093609 574.151525 440.673511 574.151525z" fill="#272636" p-id="5923"></path></svg>
<!--      <label v-if="pairState" class="pair-true">配对成功.</label>-->
    </div>
  </div>
  <div>
    <div class="device-info-container">
      <DeviceInfoContainer v-if="selfInfo" :is-self=true :device-info="selfInfo" ></DeviceInfoContainer>
      <el-scrollbar class="device-infos">
        <DeviceInfoContainer
            v-for="(info, index) in devices"
            :key="index"
            :is-self=false
            :device-info="info"
            class="device-info"
            :class="{ 'zebra-stripe-odd': isOdd(index), 'zebra-stripe-even': isEven(index) }"
        ></DeviceInfoContainer>
      </el-scrollbar>
    </div>
<!--    <el-divider />-->

    <div class="setting">
      <el-switch
          v-model="pin"
          class="ml-2"
          inline-prompt
          style="--el-switch-on-color: #13ce66; --el-switch-off-color: #a6adb6"
          active-text="钉在桌面"
          inactive-text="钉在桌面"
      />
      <el-switch
          v-model="autoStart"
          class="ml-2"
          inline-prompt
          style="--el-switch-on-color: #13ce66; --el-switch-off-color: #a6adb6"
          active-text="开机自启动"
          inactive-text="开机自启动"
      />
      <el-switch
          v-model="canTray"
          class="ml-2"
          inline-prompt
          style="--el-switch-on-color: #13ce66; --el-switch-off-color: #a6adb6"
          active-text="关闭到托盘"
          inactive-text="关闭到托盘"
      />
    </div>
  </div>
<!--  <button @click="change()">动画</button>-->
<!--  <el-button plain @click="openNotification(`success`,`配对成功`)"> Success </el-button>-->
</template>

<script setup>
import {ref, nextTick, watch, onMounted,h} from 'vue';
import {invoke} from "@tauri-apps/api/tauri";
import {listen} from "@tauri-apps/api/event";
import DeviceInfoContainer from "./components/DeviceInfoContainer.vue";
import {appWindow} from "@tauri-apps/api/window";
import {writeText} from "@tauri-apps/api/clipboard";
import {disable, enable} from "tauri-plugin-autostart-api";
function change(){
  loading.value = !loading.value
  openNotification("info","woce")
  console.log(selfInfo.value)
  devices.value.push({
    device_name:"XBss",
    device_type:"Phone",
    socket_addr:"11111.111.11"
  })
  // ElMessageBox.alert('服务器错误！', '', {
  ElMessageBox.alert(h(
      'a',
      {
        style: 'color:white; font-size: 25px; font-weight: bold;',
      },
      `服务器错误!!!`
    ), '', {
    // if you want to disable its autofocus
    showConfirmButton:false,
    confirmButtonText: '',
    // showClose:false,
    center:true,
    customClass:"server-alert"
  })
}
async function copy(){
  if (pairState.value){
    let text = pairCode.value.join("");
    await writeText(text);
    openNotification("success","成功复制配对码！")
  }
}
const loading = ref(false)
const pairState = ref(false);
const pairCode = ref(['', '', '', '', '', '']);
const selfInfo = ref(null)
const devices = ref([]);
const serverError = ref(false);
const autoStart = ref(JSON.parse(localStorage.getItem('auto-start')) || true)
const pin = ref(JSON.parse(localStorage.getItem('pin')) || false)
const canTray = ref(JSON.parse(localStorage.getItem('can-tray')) || true)
let success = false
onMounted(async ()=>{
  invoke('start_listen', {}).then()
  //下面用了两个方法来获取设备信息，只要有一个能获取到就可以
  invoke('get_self_info', {}).then((data)=>{
    //这个有时会ui初始化太快后端还没准备好消息
    if (data!=null&&!success){
      selfInfo.value = data
      openNotification("success","获取设备信息成功!")
      success = true
    }
    // if (data==null){
    //   // serverError.value = true
    // }else {
    //   selfInfo.value = data
    //   openNotification("success","获取设备信息成功!")
    // }
  })
  await listen('self_info',(event)=>{
    //这个有时会后端初始化太快但是ui的轮询消息还没开始
    if (!success){
      console.log(`设备信息${event.payload}`)
      selfInfo.value = event.payload
      openNotification("success","获取设备信息成功!")
      success = true
    }
  })
  setTimeout(function() {
    //如果五秒后还没有获取到设备信息则服务器异常
    if (!success){
      serverError.value = true
    }
  }, 5000);
  await appWindow.setAlwaysOnTop(pin.value)
  if (autoStart.value){
    await enable();
  }else{
    await disable();
  }
  await invoke('exit_setting',{canTray:canTray.value})
  await listen('no_pair_device',()=>{
    console.log("收到no_pair_device")
    loading.value = false
    openNotification("error","没有配对的设备!")
  })
  await listen('pair_code',(event)=>{
    const code = event.payload
    console.log("收到pair_code")
    console.log(code)
    loading.value = false
    const charArray = code.split('')
    pairCode.value = charArray
    pairState.value = true
    openNotification("success","创建配对码成功!")
  })
  await listen('pair_devices',(event)=>{
    pairState.value = true
    loading.value = false
    devices.value = event.payload;
    openNotification("info",`已添加设备${info.device_name}`)
  })
  await listen('add_device',(event)=>{
    let info = event.payload;
    devices.value.push(info)
    openNotification("info",`已添加设备${info.device_name}`)
  })
  await listen('remove_device',(event)=>{
    let info = event.payload;
    let index = devices.value.findIndex(item => {
      return item.socket_addr === info.socket_addr;
    });
    if (index !== -1) {
      devices.value.splice(index, 1);
    }
    openNotification("info",`已移除设备${info.device_name}`)
  })
  await listen('remove_self',()=>{
    pairState.value = false
    pairCode.value = ['', '', '', '', '', '']
    devices.value = []
    openNotification("error",`您已被取消配对！`)
  })
  await listen('server_error',()=>{
    console.log("服务器错误")
    serverError.value = true
  })

})
watch(pin, async (newVal, oldVal) => {
  localStorage.setItem("pin",newVal.toString())
  await appWindow.setAlwaysOnTop(newVal)
  // console.log(JSON.parse("false"))
  // console.log(JSON.parse(localStorage.getItem("asd"))||true)
});
watch(autoStart, async (newVal, oldVal) => {
  localStorage.setItem("auto-start",newVal.toString())
  if (newVal){
    await enable();
  }else{
    await disable();
  }
  // console.log(JSON.parse("false"))
  // console.log(JSON.parse(localStorage.getItem("asd"))||true)
});
watch(canTray, async (newVal, oldVal) => {
  localStorage.setItem("can-tray",newVal.toString())
  await invoke('exit_setting',{canTray:newVal})
  // console.log(JSON.parse("false"))
  // console.log(JSON.parse(localStorage.getItem("asd"))||true)
});
watch(serverError, (newVal, oldVal) => {
  if (newVal){
    ElMessageBox.alert(h(
        'a',
        {
          style: 'color:white; font-size: 25px; font-weight: bold;',
        },
        `服务器错误!!!`
    ), '', {
      // if you want to disable its autofocus
      showConfirmButton:false,
      confirmButtonText: '',
      showClose:false,
      center:true,
      customClass:"server-alert"
    })
  }
});
async function pairCreateRequest(){
  loading.value = true
  await invoke('pair_create',{})
}
function isOdd(index) {
  return index % 2 === 0; // 注意这里我们使用0表示偶数，1表示奇数
}
// 判断索引是否为偶数
function isEven(index) {
  return index % 2 === 1; // 注意这里我们使用1表示奇数，0表示偶数
}
function handleInput(index, event){
  const value = event.target.value;
  // 如果输入的值长度大于1，则截断为一位数字
  if (value.length > 1) {
    event.target.value = value.slice(-1);
  }
  pairCode.value[index] = event.target.value;
  // 自动跳到下一个输入框
  if (value && index < pairCode.value.length - 1) {
    const nextInput = event.target.nextElementSibling;
    if (nextInput) {
      nextTick(() => {
        nextInput.focus();
      });
    }
  }else if (value && index === pairCode.value.length - 1){
    console.log(pairCode.value.join(""))
    loading.value = true
    invoke('pair_request', { code: pairCode.value.join("") }).then(() =>
        console.log()
    )
  }
}

function handleKeyDown(index, event){
  // 处理删除操作
  if (event.key === 'Backspace' && !event.target.value && index > 0) {
    const prevInput = event.target.previousElementSibling;
    if (prevInput) {
      nextTick(() => {
        prevInput.focus();
      });
    }
  }
}
function openNotification(type,text){
  ElNotification({
    title: '',
    message: h('div', { style: 'height:auto;' }, [
      h(
          'a',
          {
            style: 'margin-top:5px;color: black; font-size: 18px; font-weight: bold;',
          },
          `${text}`
      )
    ]),
    type: `${type}`,
    customClass:`custom-notification-${type}`
  })
}

const svg = `
        <path class="path" d="
          M 30 15
          L 28 17
          M 25.61 25.61
          A 15 15, 0, 0, 1, 15 30
          A 15 15, 0, 1, 1, 27.99 7.5
          L 15 15
        " style="stroke-width: 4px; fill: rgba(0, 0, 0, 0)"/>
      `
</script>
<style>
.custom-notification-error {
  background-color: #f56c6c70!important;
  width: 300px!important;
}
.custom-notification-success {
  background-color: rgba(115, 199, 126, 0.7) !important;
  width: 300px!important;
  height:auto!important;

}
.custom-notification-waring {
  background-color: rgba(220, 188, 33, 0.7)!important;
  width: 300px!important;
}
.custom-notification-info {
  background-color: rgba(187, 179, 119, 0.7)!important;
  width: 300px !important;
}
.server-alert{
  background-color: #e34f4f !important;
}
</style>
<style lang="scss" scoped>
.pair-title{
  font-size: 20px;
  font-weight: bold;
  margin-left: 30px;
}
.verification-container {
  display: flex;
  margin-top: 10px;
  margin-left: 6px;
}

.verification-input {
  width: 10px;
  height: 20px;
  margin-right: 8px;
  text-align: center;
  font-size: 18px;
  font-weight: bold;
  border: 1px solid #ccc;
  border-radius: 5px;
}

.verification-input:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 5px #007bff;
  caret-color: transparent;
}
/*隐藏上下改变值的*/
.no-spinners::-webkit-inner-spin-button,
.no-spinners::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}
.no-spinners {
  -moz-appearance: textfield; /* Firefox 支持 */
}
.pair-state{
  display: flex;
  justify-content: center;
  align-items: center;
  margin-top: 10px;
}
.icon{
  height: 16px;
  width: 16px;
  margin-left: 10px;
  margin-top: 5px;
}
.icon:hover{
  cursor: pointer;
}
.pair-false{
  font-size: 15px;
  color: red;
  border: none;
  background-color: transparent;
}
.pair-true{
  font-size: 15px;
  color: green;
  border: none;
  background-color: transparent;
}
.pair-true:hover{
  cursor:default;
}
.device-info-container {
  margin-top: 30px;
}
.device-infos{
  margin-top: 15px;
  height: 330px;
  margin-right: 30px;
}
.device-info{
  margin-bottom: 5px; /* 设置底部间距为 20px */
}
/* 定义斑马纹的背景色 */
.device-info.zebra-stripe-odd {
  background-color: #f9f9f9;
}

.device-info.zebra-stripe-even {
  background-color: #7ed39d30;
  border-radius: 5px;
}
.setting{
  margin-left: 15px;
  margin-top: 5px;
}
.setting > * {
  margin-left: 15px;
}
</style>