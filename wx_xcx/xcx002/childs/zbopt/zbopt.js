const appObj = getApp()
Component({
  data: {
    mtype:'0',  // 0视频趋势 1商品趋势
    zbUrl: '',
    selGrp: '',
    opts: [{text:'请选择分组',value:''}]
  },
  methods: {
    onLoad() {
      if(!appObj.globalData.reqtoken) {
        wx.switchTab({
           url: '/mine/mine'
        })
        return
      }
      let thObj = this
      wx.request({
        method: 'GET',
        url: appObj.globalData.apiBaseUrl + 'api/ds/Zbgroup/Getgroups',
        header: {
          'Content-Type':'application/json',
          'Authorization': `Bearer ${appObj.globalData.reqtoken}` //'Authorization':'Bearer '+wx.getStorageSync('userToken').access_token,
        },
        success(res) {
          if(res.data.Data) {
            let tmpObj = [{text:'请选择分组',value:''}]
            res.data.Data.forEach(el => {
              tmpObj.push({
                 text:el.label,
                 value:el.value
              })
            })
            thObj.setData({
               opts:tmpObj
            })
          }
        },
        fail(err) {
          console.log(err)
        }
      })
    },
    inputChg ({detail}) {
      this.setData({zbUrl:detail})
    },
    chgHandle({detail}) {
      this.setData({selGrp:detail})
    },
    doAddZb() { // 添加达人
      if(!this.data.zbUrl) {
         wx.showToast({
            title: '请输入链接地址',
            icon: 'error'
         })
         return
      }
      let thObj = this
      if(!this.data.selGrp) this.data.selGrp = ''
      if(this.data.zbUrl.indexOf('https')==-1) {
        wx.showToast({
           title: '链接地址格式不正确',
           icon: 'error'
        })
        return
      }
      wx.request({
        method: 'POST',
        url: appObj.globalData.apiBaseUrl + 'api/ds/ZbInfo/OptAsync',
        header: {
          'Content-Type':'application/json',
          'Authorization': `Bearer ${appObj.globalData.reqtoken}` //'Authorization':'Bearer '+wx.getStorageSync('userToken').access_token,
        },
        data: {
          url: this.data.zbUrl,
          gid: [this.data.selGrp]
        },
        success(res) {
          thObj.setData({zbUrl:'', selGrp:''})
          let isSucc = res.data.IsSuccess
          if(!isSucc && res.data.Msg) {
            wx.showToast({
              title: res.data.Msg,
              icon: 'error'
            })
            return
          }
          if(isSucc) wx.showToast({
            title: '添加成功',
            icon: 'success'
          })
        },
        fail(err) {
          // console.log(err)
          thObj.setData({zbUrl:'', selGrp:''})
        }
      })  
    }
  }
})
