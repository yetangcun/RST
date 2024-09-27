import axiosReq from '../utils/axiosUtil'

// 系统接口
export const systemReq = new axiosReq({  // system 模块的request实例
    // baseURL: 'http://localhost:5298/',
    //baseURL: 'http://192.168.30.166:8686/', http://192.168.31.120:8686/
    baseURL: import.meta.env.VITE_BASE_URL, // 'http://192.168.30.166:5298/',
    // baseURL: 'http://yao159.f3322.net:6116/',
    // baseURL: 'http://asr.7766.org:6116/',
    withCredentials: false,
    timeout: 30000,
    headers: {
      'Content-Type': 'application/json;charset=utf-8'
    }
  })

  // 黑名单接口
  // export const blackReq = new axiosReq({
  //   baseURL: 'http://localhost:5298/',
  //   withCredentials: false,
  //   timeout: 30000,
  //   headers: {
  //     'Content-Type': 'application/json;charset=utf-8'
  //   }
  // })