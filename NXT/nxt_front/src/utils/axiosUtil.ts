import axios from 'axios'
import type { InternalAxiosRequestConfig, AxiosResponse, AxiosInstance } from 'axios'
import type { AxiosReqConf } from './interceptorUtil'
// import { Modal, message } from 'ant-design-vue'
import { ElMessage, ElMessageBox } from 'element-plus'
import router from '@/router'

class axiosReq {
  static runCounts = 0 // 超时重新登录,重复响应处理方法
  // static ENV:string = 'dev' // 线上、线下运行环境
  reqIns: AxiosInstance

  constructor(conf: AxiosReqConf) {
    this.reqIns = axios.create(conf)
    if (conf.interceptors == null || !conf.interceptors.requestInterceptorHandle)
      this.reqIns.interceptors.request.use(
        (req: InternalAxiosRequestConfig) => {
          const token = localStorage.getItem('curraccesstken')
          if (token) req.headers!.Authorization = `Bearer ${token}`
          else if (req.url != 'api/system/User/LgHdleAsync') router.replace('/')
          return req
        },
        (err: any) => err
      )
    else
      this.reqIns.interceptors.request.use(
        conf.interceptors.requestInterceptorHandle,
        conf.interceptors.requestInterceptorCatch
      )

    if (conf.interceptors == null || !conf.interceptors.responseInterceptorHandle)
      this.reqIns.interceptors.response.use(
        (res: AxiosResponse) => {
          const data = res.data
          if (data.Code == 401) {
            router.replace('/')
            axiosReq.runCounts += 1
            if (axiosReq.runCounts < 2) {
              ElMessageBox.alert('登录超时,请重新登录!', '登录超时', {
                confirmButtonText: '确定'
              })
            }
            return
          }

          const newToken = res.headers['fresh_token']
          if (newToken) localStorage.setItem('curraccesstken', newToken)

          return res.data
        },
        (err) => {
          if (err.code == 'ERR_NETWORK') {
            // ElMessage({
            //   message: '网络错误, 请检查网络连接和后台服务是否运行!',
            //   type: 'error'
            // })
            ElMessage.error('网络错误, 请检查网络连接和后台服务是否运行!')
            return
          }
          if (err.code != 'ERR_BAD_RESPONSE' && err.response.status == 401) {
            router.replace('/')
            axiosReq.runCounts += 1
            if (axiosReq.runCounts < 2) {
              // ElMessageBox.alert('登录超时,请重新登录', '登录超时提醒', {
              //   confirmButtonText: '确定'
              // })
              ElMessageBox({
                type: 'warning',
                title: '登录超时提醒',
                message: '登录超时,请重新登录',
                showCancelButton: false,
                confirmButtonText: '确定',
                callback: () => {}
              })
            }
            return
          }
          return err
        }
      )
    else
      this.reqIns.interceptors.response.use(
        conf.interceptors.responseInterceptorHandle,
        conf.interceptors.responseInterceptorCatch
      )
  }
}

export default axiosReq
