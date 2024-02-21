Component({
  pageLifetimes: {
    data: {
      mtype:0,  // 0视频趋势 1商品趋势
    },
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
