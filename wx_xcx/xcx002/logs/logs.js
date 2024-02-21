// logs.js
const util = require('../utils/util.js')

Component({
  data: {
    logs: []
  },
  lifetimes: {
    attached() {
      this.setData({
        logs: (wx.getStorageSync('logs') || []).map(log => {
          return {
            date: util.formatTime(new Date(log)),
            timeStamp: log
          }
        })
      })
    }
  },
  pageLifetimes: {
    show() {
      if (typeof this.getTabBar === 'function') {
        this.getTabBar((tabBar) => {
          tabBar.setData({
            selected: 2
          })
        })
      }
    }
  }
})
