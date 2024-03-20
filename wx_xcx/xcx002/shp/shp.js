let appObj = getApp()
Component({
  data: {
    mtype:'0',  // 0视频趋势 1商品趋势
    fltTitle: '筛选条件',
    dtTxt: '观测时间',
    query: '',
    dtRange: '',
    showCal:false,
    maxCal:0,
    minCal:0,
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
      let fd = 'sales'
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
      console.log(dta)
      wx.request({
        method: 'POST',
        data: dta,
        header: {
          'Content-Type':'application/json',
          'Authorization':`Bearer ${appObj.globalData.reqtoken}` //'Authorization':'Bearer '+wx.getStorageSync('userToken').access_token,
        },
        url:appObj.globalData.apiBaseUrl+'api/ds/Prd/GetprdPage',
        success (res) {
          console.log(res)
          thObj.setData({shpdata:res.data.Datas,islding:false}) // console.log(thisObj.data.shpdata)
        },
        fail (err) {
          console.log(err)
        }
      })
    },
    onShpQuery () {
      this.selectComponent("#item1").toggle()
      this.reloadOpt()
    },
    onDtDisplay (event) {
      this.setData({showCal:true})
    },
    onShpConfirm(event) {
      if(!event.detail) return
      const [start, end] = event.detail;
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
      let minTimes = sdt.getTime() - 2592000000  // 往前推一个月
      this.setData({dtRange:`${sdate}-${edate}`, maxCal:maxTimes, minCal:minTimes})
    },
    onLoad () {
      this.initTmRg()
    },
    onPullDownRefresh () { // 下拉刷新

    },
    onReachBottom () { // 上拉加载

    }
  }
})
