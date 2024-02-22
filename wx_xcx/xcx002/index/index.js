// index.js
const defaultAvatarUrl = 'https://mmbiz.qpic.cn/mmbiz/icTdbqWNOwNRna42FI242Lcia07jQodd2FJGIYQfG0LAJGFxM4FbnQP6yfMxBgJ0F3YRqJCJ1aPAK2dQagdusBZg/0'
const appObj = getApp()
Component({
  data: {
    motto: '',
    userInfo: {
      avatarUrl: defaultAvatarUrl,
      nickName: '',
      openid: '',
      access_token:  ''
    },
    items: [],
    hasUserInfo: false,
    canIUseGetUserProfile: wx.canIUse('getUserProfile'),
    canIUseNicknameComp: wx.canIUse('input.type.nickname'),
  },
  pageLifetimes: {
    show() {
      if (typeof this.getTabBar === 'function') {
        this.getTabBar((tabBar) => {
          tabBar.setData({
            selected: 0
          })
        })
      }
    }
  },
  methods: {
    // 事件处理函数
    bindViewTap() {
      wx.navigateTo({
        url: '../logs/logs'
      })
    },
    swtichPG(e) {
      if(!appObj.globalData.reqtoken) {
        wx.switchTab({
          url: '/mine/mine',
        })
        return
      }
      const id = e.target.dataset.optp
      let url = ''
      switch(id) {
        case '1': url='../childs/zb/zb'; break;
        case '2': url='../childs/vd/vd'; break;
        case '3': url='../childs/zbopt/zbopt'; break;
      }
      wx.navigateTo({
        url: url,
      })
    },
    onChooseAvatar(e) {
      const { avatarUrl } = e.detail
      const { nickName } = this.data.userInfo
      this.setData({
        "userInfo.avatarUrl": avatarUrl,
        hasUserInfo: nickName && avatarUrl && avatarUrl !== defaultAvatarUrl,
      })
    },
    onInputChange(e) {
      const nickName = e.detail.value
      const { avatarUrl } = this.data.userInfo
      this.setData({
        "userInfo.nickName": nickName,
        hasUserInfo: nickName && avatarUrl && avatarUrl !== defaultAvatarUrl,
      })
    },
    getUserProfile(e) {
      wx.login({
        success: (res) => { //可以 得到code
          // 根据 code 得到 opendid: 
          // https://api.weixin.qq.com/sns/jscode2session?appid=wx2381b1e44ebfa7b3&secret=cb084353216a01924c7b34e7cf3708f7&js_code=0b3z3y1w33yki23O9t1w3f1zMw2z3y18&grant_type=authorization_code
          const openid  = 'o6sK16-YVMRtHhynTJLNRuCgAwIs'
          this.getAccessToken().then((access_token)=>{ // 得到access_token
            this.data.userInfo.access_token = access_token
            this.data.userInfo.openid = openid
            // console.log(this.data.userInfo)
          }).catch((err) => {
            console.error('获取access_token失败', err);
          });
        },
      })
      // 推荐使用wx.getUserProfile获取用户信息，开发者每次通过该接口获取用户个人信息均需用户确认，开发者妥善保管用户快速填写的头像昵称，避免重复弹窗
      // wx.getUserProfile({
      //   desc: '展示用户信息', // 声明获取用户个人信息后的用途，后续会展示在弹窗中，请谨慎填写
      //   success: (res) => {
      //     //console.log(res)
      //     this.setData({
      //       userInfo: res.userInfo,
      //       hasUserInfo: true
      //     })
      //   }
      // })
    },
    getSelfDatas() {
      wx.request({
        url: 'http://localhost:5121/WeatherForecast',
        data: null,//data,
        //dataType: 'json',//dataType,
        enableCache: false, //true,
        enableChunked: false,
        enableHttp2: true,
        enableHttpDNS: false,
        enableQuic: true,
        forceCellularNetwork: false,
        header: {
          'Content-Type':'application/json'
        },
        httpDNSServiceId: 'httpDNSServiceId',
        method: 'get',
        responseType: undefined,//responseType,
        timeout: 0,  // 毫秒
        success: (result) => {
          let datas = result.data
          if(!datas || datas.length<1)  return
          let lens =  datas.length
          let temArrs = []
          for(let i=0;i<lens;i++)  {
             let dt = datas[i]
             temArrs.push({
               date:dt.date,
               feel:dt.summary,
               min:dt.temperatureC,
               max:dt.temperatureF
             })
          }
          this.setData({items:temArrs})
          // console.log(this.data.items, lens)
        },
        fail: (err) => {
          console.log(err)
        },
        complete: (res) => {
          //console.log(res)
        }
      })
    },
    getAccessToken() {
      const url = `https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid=wx2381b1e44ebfa7b3&secret=cb084353216a01924c7b34e7cf3708f7`;
      return new Promise((resolve, reject) => {
        wx.request({
          url: url,
          method: 'GET',
          success: function(res) {
            resolve(res.data.access_token);
          },
          fail: function(err) {
            reject(err);
          }
        });
      });
    },
    sendTemplateMessage() {
      let  uinfos = this.data.userInfo
      let tempIds =  ['5FGe95VUQ8wZ7XSHI3OknyAEOjphFomZ3I2GlQ-w51I']
      wx.requestSubscribeMessage({  // 弹窗提示授权
        tmplIds: tempIds,
        success(res) {
            const keyArr = Object.keys(res)
            const optType = res[keyArr[0]] // console.log(optType)
            if(optType==='reject') {
              wx.showToast({
                title: '取消授权',
                icon: 'error'
              })
              return
            }

            const url = `https://api.weixin.qq.com/cgi-bin/message/subscribe/send?access_token=${uinfos.access_token}`;
            const datas = {
              thing1: {value:'达人新增点赞排名'},
              thing2: {value:'最新排名'},
              thing4: {value:'通知类'},
              thing5: {value:'爆品及时达|爆品及时达|爆品及时达'}
            }

            const postData = {
              touser: uinfos.openid, // openid,
              template_id: '', // templateId, 5FGe95VUQ8wZ7XSHI3OknyAEOjphFomZ3I2GlQ-w51I
              data: datas //data
            };
    
            tempIds.forEach(tempid => {
                postData.template_id = tempid
                wx.request({
                    url: url,
                    method: 'POST',
                    data: postData,
                    success: function(res) {
                      console.log('发送模板消息成功', res);
                      if(res.data && res.data.errmsg!='ok') {
                        wx.showToast({
                          title: '授权失败',
                          icon: 'error'
                        })
                        return
                      }
                      wx.showToast({
                        title: '授权成功',
                        icon: 'success'
                      })
                    },
                    fail: function(err) {
                      console.error('发送模板消息失败', err);
                    }
                });
            });
        },
        fail (err) {
          console.log(err)
          alert(err.errMsg)
        }
      })
      
    }
  }
})
