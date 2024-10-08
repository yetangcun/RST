let appObj = getApp()
Component({
  data: {
    mtype:'0',  // 0视频趋势 1商品趋势
    fltTitle: '筛选',
    dtTxt: '观测时间',
    query: '',
    dtRange: '',
    showCal:false,
    maxCal:0,
    minCal:0,
    islding: false,
    shpdata: []
  },
  methods: {
    switchNav(e) {
      const mt = e.target.id
      if(mt!=this.data.mtype) this.setData({mtype:mt})
    },
    reloadOpt () {
      if(!appObj.globalData.reqtoken) {
        wx.switchTab({
          url: '/mine/mine',
        })
        wx.showToast({
          title: '请先登录',
          icon: 'error'
        })
        return
      }
      let thObj = this
      let upTm = this.data.dtRange.split('-')
      let fd = 'sales' // 写定为按照销量排序 
      let dta = {
        clttype: 1,
        PageSize: 100,
        CurrentIndex: 1,
        ReqParams: {
          field: fd,
          times: upTm,
          content: this.data.query
        }
      }
      this.setData({islding:true})
      wx.request({
        method: 'POST',
        data: dta,
        header: {
          'Content-Type':'application/json',
          'Authorization':`Bearer ${appObj.globalData.reqtoken}` //'Authorization':'Bearer '+wx.getStorageSync('userToken').access_token,
        },
        url:appObj.globalData.apiBaseUrl+'api/ds/Prd/GetprdPage',
        success (res) {
          res.data.Datas.forEach(s=>{
            s.price = (s.price/100).toFixed(1)
          })
           thObj.setData({shpdata:res.data.Datas,islding:false}) // console.log(thisObj.data.shpdata)
        },
        fail (err) {
          this.setData({islding:false})
        }
      })
    },
    onShpQuery () {
      this.selectComponent("#item1").toggle()
      this.reloadOpt()
    },
    onDtDisplay (e) {
      console.log(e)
      this.setData({showCal:true})
    },
    onShpConfirm(e) {
      if(!e.detail) return
      const [start, end] = e.detail;
      this.setData({
         showCal: false,
         dtRange: `${this.formatDate(start)}-${this.formatDate(end)}`,
      });
    },
    formatDate(date) {
      date = new Date(date);
      return `${date.getMonth() + 1}.${date.getDate()}`;
    },
    initTmRg() {
      let dte = new Date(); dte = dte.setDate(dte.getDate()-2)
      let sdt = new Date(dte)
      let mt = sdt.getMonth() + 1  // 起始观测日期
      let dt = sdt.getDate()
      
      let edte = new Date()          // 发布、观测截至日期
      let emt = edte.getMonth() + 1
      let edt = edte.getDate()

      let sdate = `${mt}.${dt}`
      let edate = `${emt}.${edt}`
      
      let maxTimes = edte.getTime() + 86400000    // 往后推一天 24*60*60*1000
      let minTimes = sdt.getTime() - 2592000000   // 往前推一个月
      this.setData({dtRange:`${sdate}-${edate}`, maxCal:maxTimes, minCal:minTimes})

      this.reloadOpt()
    },
    inputAccpt({detail}) {
      this.setData({query: detail})
    },
    onPullDownRefresh () { // 下拉刷新

    },
    onReachBottom () { // 上拉加载

    }
  },
  lifetimes: {
    attached ()  {
      if(!appObj.globalData.reqtoken) {
        wx.switchTab({
          url: '/mine/mine',
        })
        return
      }
      this.initTmRg()
    }
  },
  pageLifetimes: {
    show() {
      if (typeof this.getTabBar === 'function') {
        this.getTabBar((tabBar) => {
          tabBar.setData({
            selected: 1
          })
        })
      }
    }
  }
})
