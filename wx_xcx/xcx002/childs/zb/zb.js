Component({
  data: {
    mtype:'0',  // 0视频趋势 1商品趋势
    filterTitle: '筛选条件'
  },
  methods: {
    switchNav(e) {
      const mt = e.target.id
      if(mt!=this.data.mtype) this.setData({mtype:mt})
    }
  }
})
