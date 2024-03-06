const appObj = getApp()
Component({
  data: {
    mtype:'1',    // 1视频趋势 2商品趋势 3达人趋势
    vdata:null,   // 视频趋势数据
    shpdata:null, // 商品趋势数据
    zbdata:null,  // 达人趋势数据
    selRankTp: '1',
    query: '',
    mind: '',
    maxd: '',
    islding:false,
    dtTxt1: '发布日期',
    dtTxt2: '观测日期',
    choose2: '达人分组',
    choose3: '排序类型',
    showCal: false,
    showGrp: false,
    showOdtp: false,
    odtps: ['新增点赞', '总点赞'],
    grps: ['无','分组1','分组2','分组3'],
    grpid: '',        // 分组类型
    dr: '',           // 达人
    odtp: '新增点赞',  // 排序类型
    dtRange1: '',     // 发布日期范围
    dtRange2: '',     // 观测日期范围
    minCal: 0,
    maxCal: 0,
    ranks: [
      { text:'视频趋势榜',value:'1'},
      { text:'商品趋势榜',value:'2'},
      { text:'达人趋势榜',value:'3'}
    ],
    itemTitle: '筛选'
  },
  methods: {
    onConfirm() {
      let tmp = `${this.data.dtRange1}-${this.data.dtRange2}-${this.data.grpid}-${this.data.dr}-${this.data.odtp}`
      console.log(tmp)
      this.selectComponent('#item2').toggle();
    },
    onCalDisplay(e) {
      let tmtp = e.target.dataset.tmtp
      if(tmtp==='1') this.setData({ showCal: true });
      if(tmtp==='2') this.setData({ showCal1: true });
    },
    onCalClose() {
      this.setData({ showCal: false });
    },
    onCalConfirm(event) {
      if(!event.detail) return
      const [start, end] = event.detail;
      let tmcal = event.target.dataset.tmcal
      if(tmcal === '1') this.setData({
        showCal: false,
        dtRange1: `${this.formatDate(start)}-${this.formatDate(end)}`,
      });
      else if(tmcal==='2') this.setData({
        showCal1: false,
        dtRange2: `${this.formatDate(start)}-${this.formatDate(end)}`,
      });
    },
    onGrpDisplay(event) {
      console.log(event)
      this.setData({ showGrp: true });
    },
    onGrpChange({detail}) {
      this.setData({grpid: detail.value})
    },
    onGrpConfirm()  {
      this.setData({ showGrp: false });
    },
    onOdTpDisplay() {
      this.setData({showOdtp:true})
    },
    onOdtpConfirm() {
      this.setData({showOdtp:false})
    },
    onOdtpChange({detail}) {
      this.setData({odtp:detail.value})
    },
    formatDate(date) {
      date = new Date(date);
      return `${date.getMonth() + 1}.${date.getDate()}`;
    },
    switchRank({detail}) {
      if(!detail) return
      if(detail!=this.data.mtype) this.setData({mtype:detail})
      console.log(detail)
      this.doLoad(detail)
    },
    reloadOpt(e) {
      const mt = e.target.dataset.datap
      this.doLoad(mt)
    },
    doLoad(tp) {
      let fd =  this.data.odtp === '总点赞'? 'dzs':'dz'
      let pubTm = this.data.dtRange1?this.data.dtRange1.split('-'):[]
      let upTm = this.data.dtRange2?this.data.dtRange2.split('-'):[]
      let dr =  this.data.dr?[this.data.dr]:[]
      let thisObj = this
      thisObj.setData({vdata:null})
      thisObj.setData({shpdata:null})
      thisObj.setData({zbdata:null, islding:true})
      
      if(tp==='1') { // 视频趋势
        if(this.vdata==null) {
          wx.request({
            method: 'POST',
            data: {
              clttype: 1,
              PageSize: 100,
              CurrentIndex: 1,
              ReqParams: {
                field: fd,
                order: '1',
                gid: this.data.grpid,
                uid: dr,
                times: upTm,
                putimes: pubTm,
                txt: this.data.query
              }
            },
            header: {
              'Content-Type':'application/json',
              'Authorization': `Bearer ${appObj.globalData.reqtoken}` //'Authorization':'Bearer '+wx.getStorageSync('userToken').access_token,
            },
            url: `${appObj.globalData.apiBaseUrl}`+'api/ds/Vediochg/GetVediochgByPage',
            success (res) {
               thisObj.setData({vdata:res.data.Datas,islding:false})
            },
            fail (err) {
              console.log(err)
            }
          })
        }
        else { }
        return
      }

      if(tp === '2') { // 商品趋势
        if(this.shpdata==null) {
          thisObj.setData({shpdata:[],islding:false})
        }
        else { }
        return
      }
      
      if(tp === '3') { // 达人趋势 
        if(this.zbdata==null) {
          thisObj.setData({zbdata:[],islding:false})
        }
        else { }
      }
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

      let dte = new Date(); dte = dte.setDate(dte.getDate()-14)
      let sdt = new Date(dte)
      let mt = sdt.getMonth() + 1  // 发布日期起始
      let dt = sdt.getDate()
      
      let dte1 = new Date(); dte1 = dte1.setDate(dte1.getDate()-2)
      let sdt1 = new Date(dte1)
      let mt1 = sdt1.getMonth() + 1  // 观测日期起始
      let dt1 = sdt1.getDate()

      let edte = new Date()
      let emt = edte.getMonth() + 1  // 发布、观测截至日期
      let edt = edte.getDate()

      let sdate = `${mt}.${dt}`
      let sdate1 = `${mt1}.${dt1}`
      let edate = `${emt}.${edt}`
      
      let maxTimes = edte.getTime() + 86400000    // 往后推一天 24*60*60*1000
      let minTimes = edte.getTime() - 2592000000  // 往前推一个月
      this.setData({dtRange1:`${sdate}-${edate}`,maxCal:maxTimes,minCal:minTimes})
      this.setData({dtRange2:`${sdate1}-${edate}`,maxCal:maxTimes,minCal:minTimes})

      this.doLoad('1')  // 默认加载
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