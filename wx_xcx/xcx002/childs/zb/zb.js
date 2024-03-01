// const { FactorGradient } = require("XrFrame/components/particle/gradient")
Component({
  data: {
    mtype:'0',  // 0视频趋势 1商品趋势
    fltTitle: '筛选条件',
    grpTxt: '分组',
    showGrp: false,
    grps: [],
    grpid: '',
    queryTxt: '',
  },
  methods: {
    switchNav(e) {
      const mt = e.target.id
      if(mt!=this.data.mtype) this.setData({mtype:mt})
    },
    onGrpDisplay() {

    },
    onGrpConfirm () {

    },
    onGrpChange () {

    }
  }
})
