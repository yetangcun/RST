let appObj = getApp()
Component({
  data: {
    grpTxt:'分组',  // 0视频趋势 1商品趋势
    fltTitle: '筛选',
    grpid: '',
    grps: [],
    grpsObj: [],
    query: '',
    grpNm: '',
    islding: false,
    showGrp: false,
    zbdata: []
  },
  methods: {
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
      let fd = 'favorited'
      let dta = {
        clttype: 1,
        PageSize: 100,
        CurrentIndex: 1,
        ReqParams: {
          field: fd,
          gid:this.data.grpid,
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
        url:appObj.globalData.apiBaseUrl+'api/ds/ZbInfo/GetZbByPage',
        success (res) {
           res.data.Datas.forEach(r=>{
             r.favorited = r.favorited.toLocaleString()
             r.follower =  r.follower.toLocaleString()
             r.count = r.count.toLocaleString()
           })
           thObj.setData({zbdata:res.data.Datas,islding:false}) // console.log(thisObj.data.zbdata)
        },
        fail (err) {
          this.setData({islding:false})
        }
      })
    },
    onZbQuery () {
      this.selectComponent("#item1").toggle()
      this.reloadOpt()
    },
    onDtDisplay (e) {
      this.setData({showGrp:true})
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
    inputAccpt({detail}) {
      this.setData({query:detail})
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
    },
    init(){
      if(!appObj.globalData.reqtoken) {
        wx.switchTab({ url: '/mine/mine'})
        return
      }  // this.initTmRg()
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
            let tmp=['无']
            let tmpObj = []
            res.data.Data.forEach(el => {
              tmp.push(el.label)
              tmpObj.push({
                lb:el.label,
                vl:el.value
              })
            })
            thObj.setData({
              grps:tmp,
              grpsObj:tmpObj
            })
            let fgrp = thObj.data.grps.find(g=>g.indexOf('小仙家人们')!=-1)
            if(fgrp) thObj.setData({grpid:fgrp})
            thObj.reloadOpt()
          }
        },
        fail(err) {
          console.log(err)
        }
      })
    },
    onGrpDisplay(event) {
      this.setData({ showGrp: true });
    },
    onGrpChange({detail}) {
      let lb = detail.value   
      let obj = this.data.grpsObj.find(g=>g.lb===lb)
      if(!obj || lb === '无')
        this.setData({grpid: '', grpNm:''})
      else if(obj) {
        this.setData({grpNm:lb})
        this.setData({grpid: obj.vl})
      }
      // console.log(this.data.grpid, this.data.grpNm)
    },
    onGrpConfirm()  {
      this.setData({ showGrp: false });
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
      this.init()
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
