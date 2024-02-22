Component({
  data: {
    mtype:'0',  // 0视频趋势 1商品趋势
    vdata:null, // 视频趋势数据
    spdata:null // 商品趋势数据
  },
  methods: {
    switchNav(e) {
      const mt = e.target.dataset.optp
      if(mt!=this.data.mtype) this.setData({mtype:mt})
      this.doLoad(mt)
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
