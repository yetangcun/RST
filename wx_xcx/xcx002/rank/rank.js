Component({
  data: {
    mtype:'0',  // 0视频趋势 1商品趋势
    vdata:null, // 视频趋势数据
    spdata:null, // 商品趋势数据
    selRankTp: '1',
    content: '',
    mind: '',
    maxd: '',
    choose1: '观测日期',
    choose2: '达人分组',
    choose3: '排序类型',
    showCal: false,
    showGrp: false,
    showOdtp: false,
    odtps: ['新增点赞', '总点赞'],
    grps: ['无','分组1','分组2','分组3'],
    grpid: '',
    odtp: '新增点赞',
    dtRange: '',
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
      let tmp = `${this.data.dtRange}-${this.data.grpid}-${this.data.odtp}`
      console.log(tmp)
      this.selectComponent('#item2').toggle();
    },
    onCalDisplay() {
      this.setData({ showCal: true });
    },
    onCalClose() {
      this.setData({ showCal: false });
    },
    onCalConfirm(event) {
      if(!event.detail) return
      const [start, end] = event.detail;
      this.setData({
        showCal: false,
        dtRange: `${this.formatDate(start)}-${this.formatDate(end)}`,
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
      this.doLoad(detail)
    },
    reloadOpt(e) {
      const mt = e.target.dataset.datap
      this.doLoad(mt)
    },
    doLoad(tp) {
      if(tp===0 && this.vdata==null) {
        return
      }

      if(this.spdata==null) {

      }
    }
  },
  lifetimes: {
    attached ()  {
      let dte = new Date()
      dte = dte.setDate(dte.getDate()-6)
      let sdt = new Date(dte)
      let edte = new Date()
      
      let yr = sdt.getFullYear()
      let mt = sdt.getMonth() + 1
      let dt = sdt.getDate()

      let eyr = edte.getFullYear()
      let emt = edte.getMonth() + 1
      let edt = edte.getDate()

      let sdate = `${mt}.${dt}`
      let edate = `${emt}.${edt}`
      
      let maxTimes = edte.getTime() + 86400000
      let minTimes = edte.getTime() - 2592000000
      this.setData({dtRange: `${sdate}-${edate}`,maxCal:maxTimes,minCal:minTimes})
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
