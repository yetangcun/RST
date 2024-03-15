Component({
  data: {
    ptype:'0',  // 0视频趋势 1商品趋势
    vdurl: ''
  },
  methods: {
    onLoad (options) {
      let vd = options.vd
      if(vd) {
        let arr = vd.split('_')
        let url = `https://www.douyin.com/user/${arr[1]}?modal_id=${arr[0]}}`
        this.setData({vdurl:url})
      }
    },
    switchNav(e) {
      const mt = e.target.id
      if(mt!=this.data.mtype) this.setData({mtype:mt})
    }
  }
})
