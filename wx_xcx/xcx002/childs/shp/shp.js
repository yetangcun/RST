Component({
  data: {
    mtype:'0',  // 0视频趋势 1商品趋势
    fltTitle: '筛选条件',
    dtTxt: '观测时间',
    dtRange: '',
    query: ''
  },
  methods: {
    switchNav(e) {
      const mt = e.target.id
      if(mt!=this.data.mtype) this.setData({mtype:mt})
    }
  }
})
