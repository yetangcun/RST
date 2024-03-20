// const { FactorGradient } = require("XrFrame/components/particle/gradient")
const appObj = getApp()
Component({
  data: {
    mtype:'0',  // 0视频趋势 1商品趋势
    fltTitle: '筛选条件',
    grpTxt: '分组',
    grpid: '',
    showGrp: false,
    grps: [],
    cnt: '',
    actKy: -1,
    grps: [],   // 分组
    grpsObj: [],
    zbs: [],
    selzbs: '',
    islding: false,
    sldstl: '.sldstl',
    showGrpOpt: false
  },
  methods: {
    switchNav(e) {
      const mt = e.target.id
      if(mt!=this.data.mtype) this.setData({mtype:mt})
    },
    onQueryConfirm () {
      this.setData({islding:true})
      this.initGrp(this.data.cnt)
      this.selectComponent("#item1").toggle()
    },
    inputAccpt(event) {
      this.setData({cnt:event.detail})
    },
    onZbChange(event) {
      if(this.data.grpid) this.setData({selzbs:event.detail,showGrpOpt:true})
      // console.log(this.data.selzbs)
    },
    saveRopt() {
      if(this.data.grpid) {
        if(!this.data.selzbs) this.data.selzbs = []
        let grp = this.data.grpsObj.find(g=>g.vl===this.data.grpid)
        let nm = grp.lb
        let thObj = this
        wx.request({
          method: 'POST',
          url: appObj.globalData.apiBaseUrl + 'api/ds/Zbgroup/OptAsync',
          header: {
            'Content-Type':'application/json',
            'Authorization': `Bearer ${appObj.globalData.reqtoken}` //'Authorization':'Bearer '+wx.getStorageSync('userToken').access_token,
          },
          data: {
            id:this.data.grpid,
            name: nm,
            uids:this.data.selzbs
          },
          success(res) {
            if(res.data.IsSuccess) {
              wx.showToast({
                title: '保存成功',
                icon: 'success'
              })
              thObj.setData({
                showGrpOpt: false
              })
              return
            }
            wx.showToast({
              title: '保存失败' + res.data.Msg,
              icon: 'error'
            })
          },
          fail(err) {
            console.log(err)
          }
        })
      }
    },
    cancelROpt() {
      this.setData({showGrpOpt:false})
    },
    initGrp (cnt) {
      if(!cnt) cnt = ''
      let thObj = this
      wx.request({
        method: 'GET',
        url: appObj.globalData.apiBaseUrl + 'api/ds/Zbgroup/Getgroups/'+cnt,
        header: {
          'Content-Type':'application/json',
          'Authorization':`Bearer ${appObj.globalData.reqtoken}` //'Authorization':'Bearer '+wx.getStorageSync('userToken').access_token,
        },
        success(res) {
          thObj.setData({islding:false})
          if(res.data.Data) {
            let tmp=['无']
            let tmpObj = []
            let ind = 0
            res.data.Data.forEach(el => {
              tmp.push(el.label)
              tmpObj.push({
                lb:el.label,
                vl:el.value,
                idx: ind++
              })
            })
            thObj.setData({
              grps:tmp,
              grpsObj:tmpObj
            })
          }
        },
        fail(err) {
          console.log(err)
        }
      })
    },
    grpchg(event) {
      let grp = this.data.grpsObj.find(g=>g.idx===event.detail)
      if(grp) {
        this.setData({grpid:grp.vl})
        let thObj = this
        wx.request({
          method: 'GET',
          url: appObj.globalData.apiBaseUrl + 'api/ds/Zbgroup/GetZbsOfGroup/' + this.data.grpid,
          header: {
            'Content-Type':'application/json',
            'Authorization': `Bearer ${appObj.globalData.reqtoken}` //'Authorization':'Bearer '+wx.getStorageSync('userToken').access_token,
          },
          success(res) {
            if(res.data.Data) {
              let tmp = []
              res.data.Data.forEach(el => {
                tmp.push(el)
              })
              thObj.setData({
                selzbs:tmp
              })
            }
          },
          fail(err) {
            console.log(err)
          }
        })
      }
    },
    initZb () {
      let thObj = this
      wx.request({
        method: 'GET',
        url: appObj.globalData.apiBaseUrl + 'api/ds/ZbInfo/Getzbinfos',
        header: {
          'Content-Type':'application/json',
          'Authorization': `Bearer ${appObj.globalData.reqtoken}` //'Authorization':'Bearer '+wx.getStorageSync('userToken').access_token,
        },
        success(res) {
          if(res.data.Data) {
            let tmp = []
            res.data.Data.forEach(el => {
              tmp.push({
                lb:el.label,
                vl:el.value
              })
            })
            thObj.setData({
              zbs:tmp
            })
          }
        },
        fail(err) {
          console.log(err)
        }
      })
    },
    reloadOpt () {
      if(!appObj.globalData.reqtoken) {
        wx.switchTab({
          url: '/mine/mine',
        })
        return
      }
      
      this.setData({selzbs:[], islding:true, showGrpOpt:false, actKy:-1, grpid:''})
      this.initZb()
      this.initGrp(this.data.cnt)
    },
    onLoad() {
      this.reloadOpt()
    }
  }
})
