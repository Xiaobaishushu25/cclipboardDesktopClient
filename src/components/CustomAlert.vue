<script setup lang="ts">
// import {types} from "sass";
// import String = types.String;
defineProps({type:String,message:String})
</script>

<template>
  <div class="custom-alert" :class="type">
    <div class="content">
      <p>{{ message }}</p>
<!--      <button @click="closeAlert">Close</button>-->
    </div>
  </div>
</template>

<style scoped>
.custom-alert {
  width: 50px;
  padding: 8px;
  /*background-color: rgb(245,108,108);*/
  color: white;
  border-radius: 8px;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
  font-size: 17px; /* Adjust font size as needed */
  line-height: 1.5; /* Adjust line height as needed */
}

.content {
  display: flex;
  flex-direction: column;
  align-items: center;
}
.info{
  background-color:dodgerblue;
}
.success{
  background-color: rgb(103,194,58);
}
.danger{
  background-color: rgb(245,108,108);
}
</style>