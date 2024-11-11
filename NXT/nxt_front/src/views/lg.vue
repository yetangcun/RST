<template>
  <div class="pg_top">
    <div class="lg_panel">
      <div
        class="panel_title"
        style="
          color: gray;
          justify-content: center;
          align-items: center;
          font-size: 26px;
          font-weight: 600;
        "
      >
        平台登录
      </div>
      <div class="panel_line">
        <User
          style="
            color: #001529;
            height: 1.5em;
            width: 1.5em;
            font-weight: 600;
            color: dodgerblue;
            padding: 1px 6px 0px 6px;
          "
        />
        <input
          type="text"
          class="input_stl"
          placeholder="请输入用户名"
          v-model="sts.lg_req.usr"
          @keydown.enter="lgHdl"
        />
      </div>
      <div class="panel_line">
        <Lock
          style="
            color: #001529;
            height: 1.5em;
            width: 1.5em;
            font-weight: 600;
            color: dodgerblue;
            padding: 1px 6px 0px 6px;
          "
        />
        <input
          type="password"
          class="input_stl"
          placeholder="请输入密码"
          v-model="sts.lg_req.pwd"
          @keydown.enter="lgHdl"
        />
      </div>
      <el-button
        type="primary"
        @click="lgHdl"
        style="width: 269px; margin-top: 26px; font-size: 18px; height: 43px"
        >登录</el-button
      >
    </div>
  </div>
</template>

<script setup lang="ts">
import { ElButton, ElMessage } from 'element-plus'
import { reactive } from 'vue'
import { useRouter } from 'vue-router'
import { dftReq } from '../utils/reqUtil'

const router = useRouter()

const sts = reactive<{
  lg_req: any
}>({
  lg_req: {
    usr: '',
    pwd: '',
    code: '999'
  }
})

// 登录处理
const lgHdl = () => {
  if (!sts.lg_req.usr) return ElMessage.warning('用户名不能为空')
  if (!sts.lg_req.pwd) return ElMessage.warning('密码不能为空')
  dftReq.reqIns
    .post('no_auth/user/dologin', sts.lg_req)
    .then((res: any) => {
      console.log(res)
      if (res.is_succ) {
        localStorage.setItem('curr_tken', res.data)
        router.replace('/main')
        return
      }
      ElMessage.error(res.msg)
    })
    .catch((err: any) => {
      console.log(err)
    })
}
</script>

<style scoped>
.pg_top {
  display: flex;
  flex: 1;
  width: 100%;
  height: 100%;
  align-items: center;
  justify-content: center;
  background-color: transparent;
}

.lg_panel {
  display: flex;
  padding: 60px 30px;
  border-radius: 36px;
  flex-direction: column;
  background-color: white;
  box-shadow: 0px 0px 19px 0px lightgray inset;
}

.panel_title {
  display: flex;
  font-weight: 600;
  color: #001529;
  margin-bottom: 16px;
  align-items: center;
  font-family: '华文楷体';
  justify-content: center;
}

.panel_line {
  display: flex;
  margin-top: 36px;
  padding: 8px 0px;
  border-radius: 26px;
  align-items: flex-end;
  border: 1px solid dodgerblue;
}

.input_stl {
  display: flex;
  border: none;
  outline: none;
  width: 217px;
  padding: 3px;
  font-size: 16px;
  font-weight: 500;
  align-items: flex-end;
  text-decoration: none;
}
</style>
