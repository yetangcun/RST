// logs.js
const util = require('../utils/util.js')
const appObj = getApp()
Component({
  data: {
    logs: [],
    islogined: 0,
    lname: '',
    lpwd: '',
    uname: '',
    uhead: ''
  },
  pageLifetimes: {
    show() {
      if (typeof this.getTabBar === 'function') {
        this.getTabBar((tabBar) => {
          tabBar.setData({
            selected: 2
          })
        })
      }
    }
  },
  methods: {
    doUserLogin() {
      if(!this.data.lname) {
        wx.showToast({
          title: '请输入登录账号',
          icon: 'error'
        })
        return
      }
      if(!this.data.lpwd) {
        wx.showToast({
          title: '请输入登录密码',
          icon: 'error'
        })
        return
      }
      let acc = util.base64Encode(this.data.lname);
      let pd = util.md5(this.data.lpwd);
      let thisObj = this
      wx.request({
        url: `${appObj.globalData.apiBaseUrl}`+'api/sys/User/LoginHandleAsync',
        method: 'POST',
        header: {
          'Content-Type':'application/json'
        },
        data: {
          Account:acc,
          Passwd:pd
        },
        success(res) {
          if(res.data && res.data.Code!=200) {
            wx.showToast({
              title: res.data.Msg,
              icon: 'error'
            })
            return
          }
          let obj = res.data;
          appObj.globalData.reqtoken = obj.Data.accessToken
          appObj.globalData.userInfo.uname = obj.Data.name
          thisObj.setData({
            islogined: 1,
            uname: obj.Data.name,
            uhead: "https://thirdwx.qlogo.cn/mmopen/vi_32/POgEwh4mIHO4nibH0KlMECNjjGxQUq24ZEaGT4poC6icRiccVGKSyXwibcPq4BWmiaIGuG1icwxaQX6grC9VemZoJ8rg/132"
          })
        },
        fail (err) {
          console.log(err)
        }
      })
    },
    bindAccInput(e) {
      this.setData({
        lname:e.detail.value
      })
    },
    bindPwdInput(e) {
      this.setData({
        lpwd:e.detail.value
      })
    },
    doWxLogin() {
      wx.showToast({
        title: '正在通过微信登录',
        icon: 'none'
      })
      let thisObj = this
      wx.login({ // 获取openid
        success: (res) => {
          const code =  res.code // 得到code
          if(!code) return
          // let appid = appObj.globalData.appid
          // let secret = appObj.globalData.secret
          // console.log(appid, secret)
          wx.request({
            url: `${appObj.globalData.apiBaseUrl}api/ds/Wxreq/GetWxloginRes/${code}`,
            method: 'GET',
            success (rs) {
              console.log(rs.data)
              if(rs.statusCode==200  && rs.data) {
                if(rs.data.openid) {
                  appObj.globalData.openid = rs.data.openid
                  appObj.globalData.reqtoken = rs.data.token
                  thisObj.setData({
                    islogined:1,
                    uname:rs.data.openid
                  })
                }
              }
            },
            fail (er) {
              console.log(er)
            }
          })

          /*
          wx.request({ // 根据appid、secret、code获取openid
            url: `https://api.weixin.qq.com/sns/jscode2session?appid=${appid}&secret=${secret}&js_code=${code}&grant_type=authorization_code`,
            method: 'GET',
            success(ress) { 
              console.log(ress)
              if(ress.data.openid) {
                appObj.globalData.openid = ress.data.openid
                thisObj.setData({
                  islogined:1
                })
              }
            },
            fail(err) {
              console.log(err)
              wx.showToast({
                title: '1' + err.errMsg,
                icon: 'error'
              })
            }
          })*/
        }
      })
      // 获取微信用户信息
      wx.getUserProfile({
        desc: '展示用户信息',
        success (resp) {
           if(resp.userInfo) {
             appObj.globalData.userInfo.uname = resp.userInfo.nickName
             appObj.globalData.userInfo.uhead = resp.userInfo.avatarUrl
             thisObj.setData({
                uname:appObj.globalData.userInfo.uname,
                uhead:appObj.globalData.userInfo.uhead
              })
           } console.log(appObj.globalData)
        },
        fail (errs) {
          console.log(errs)
          wx.showToast({
            title: '2' + err.errMsg,
            icon: 'error'
          })
        }
      })
    }
  },
  lifetimes: {
    attached() {
      // console.log(appObj.globalData)
      if(appObj.globalData.userInfo && appObj.globalData.reqtoken)  {
        this.setData({islogined:appObj.globalData.islogined,uinfos:appObj.globalData.userInfo})
      }
      // this.setData({
      //   logs: (wx.getStorageSync('logs') || []).map(log => {
      //     return {
      //       date: util.formatTime(new Date(log)),
      //       timeStamp: log
      //     }
      //   })
      // })
    }
  }
})
