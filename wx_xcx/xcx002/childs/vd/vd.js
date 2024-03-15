Component({
  data: {
    mtype:'0',  // 0视频趋势 1商品趋势
    fltTitle: '筛选条件',
    dtTxt1: '发布时间',
    dtTxt2: '观测时间',
    grpTxt:  '分组',
    dtRange1: '',
    dtRange2: '',
    grpid: '',
    dr: '',
    query: ''
  },
  methods: {
    switchNav(e) {
      const mt = e.target.id
      if(mt!=this.data.mtype) this.setData({mtype:mt})
    },
    reloadOpt () {

    }
  }
})
