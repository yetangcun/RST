<template>
  <div class="pg_top">
    <div
      class="pg_lft"
      id="pg_lft_nav"
      :style="{
        display: 'flex',
        minWidth: states.dftWdth,
        maxWidth: states.dftWdth,
        backgroundColor: states.dftBack
      }"
    >
      <div class="nav_ico">
        <span
          :class="`iconfont ${states.dftIco}`"
          :style="{ color: states.dftColor, fontSize: '54px' }"
        ></span>
        <!-- <img src="/favicon.ico" style="height: 46px; width: 46px" /> -->
      </div>
      <div class="nav_menu">
        <div v-for="(item, index) in states.menus" :key="index">
          <div class="menu_head" @click="navClkHdl(item)">
            <div class="menu_item_head">
              <span
                :class="`iconfont ${item.icon}`"
                :style="{
                  color: item.isSelected && item.mtype == 1 ? states.selColor : states.dftColor,
                  display: 'flex',
                  fontSize: item.size,
                  margin: '2px 2px 0px 0px'
                }"
              ></span>
              <div
                :key="index"
                :style="{ display: states.isTxt ? 'flex' : 'none', fontSize: '17px' }"
              >
                <span
                  v-if="item.mtype == 1 && item.isSelected"
                  :style="{ color: states.selColor }"
                  >{{ item.name }}</span
                >
                <span v-else :style="{ color: states.dftColor }">{{ item.name }}</span>
              </div>
            </div>
            <span
              v-if="item.mtype == 0 && states.isTxt"
              :class="
                item.isChildVisible ? 'iconfont icon-jiantoushang' : 'iconfont icon-jiantouxia'
              "
              :style="{ margin: '2px 11px 0px 0px', color: states.dftColor }"
            ></span>
          </div>
          <transition name="sld">
            <div v-if="item.isChildVisible" :style="{ backgroundColor: states.dftBack }">
              <div
                v-for="(child, index) in item.childs"
                :key="index"
                @click="navClkHdl(child)"
                class="sub_menu_item"
              >
                <span
                  :class="`iconfont ${child.icon}`"
                  :style="{
                    display: 'flex',
                    fontSize: child.size,
                    margin: '0px 2px 0px 11px',
                    color: child.isSelected ? states.selColor : states.dftColor
                  }"
                ></span>
                <div
                  :key="index"
                  :style="{
                    fontSize: '15px',
                    display: states.isTxt ? 'block' : 'none',
                    color: child.isSelected ? states.selColor : states.dftColor
                  }"
                >
                  {{ child.name }}
                </div>
              </div>
            </div>
          </transition>
        </div>
      </div>
      <div class="nav_foot" @click="expandHdl">
        <span
          :class="
            states.isTxt
              ? 'iconfont icon-shuangzhongjiantou'
              : 'iconfont icon-shuangzhongjiantouyou'
          "
          :style="{ color: states.dftColor, fontSize: '23px' }"
        ></span>
      </div>
    </div>
    <div class="pg_rgt">
      <div class="main_mds" :style="{ backgroundColor: states.dftBack }"></div>
      <div class="main_bdy"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, nextTick } from 'vue'
import { ElMessage } from 'element-plus'
import { animalutil } from '../utils/animal'

//#001f3c
const states = reactive({
  isAnimal: true,
  dftBack: '#001f3c',
  dftColor: 'lightgray',
  dftIco: 'icon-jingying',
  selColor: 'greenyellow',
  dftWdth: '211px',
  selCode: '',
  isTxt: true,
  menus: [
    {
      name: '系统管理',
      icon: 'icon-pc',
      code: 'systemmanager',
      size: '20px',
      mtype: 0,
      isSelected: false,
      isChildVisible: true,
      childs: [
        {
          name: '用户管理',
          icon: 'icon-pc',
          code: 'user',
          size: '18px',
          mtype: 1,
          isSelected: false,
          childs: []
        },
        {
          name: '权限管理',
          icon: 'icon-pc',
          code: 'permission',
          size: '18px',
          mtype: 1,
          isSelected: false,
          childs: []
        },
        {
          name: '菜单管理',
          icon: 'icon-pc',
          code: 'menu',
          size: '18px',
          isSelected: false,
          mtype: 1,
          childs: []
        },
        {
          name: '组织架构',
          icon: 'icon-pc',
          code: 'org',
          size: '18px',
          isSelected: false,
          mtype: 1,
          childs: []
        },
        {
          name: '系统配置等统一设置',
          icon: 'icon-pc',
          code: 'setting',
          size: '18px',
          isSelected: false,
          mtype: 1,
          childs: []
        },
        {
          name: '系统日志',
          icon: 'icon-pc',
          code: 'syslog',
          size: '18px',
          isSelected: false,
          mtype: 1,
          childs: []
        }
      ]
    },
    {
      name: '黑名单管理',
      icon: 'icon-pc',
      code: 'blacklist',
      size: '20px',
      mtype: 0,
      isSelected: false,
      isChildVisible: false,
      childs: [
        {
          name: '号码过滤',
          icon: 'icon-pc',
          code: 'blacklist',
          isSelected: false,
          size: '18px',
          mtype: 1,
          childs: []
        },
        {
          name: '黑名单规则',
          icon: 'icon-pc',
          isSelected: false,
          code: 'blacklistrule',
          size: '18px',
          mtype: 1
        }
      ]
    },
    {
      name: '智能语音',
      icon: 'icon-pc',
      code: 'intelligent',
      size: '20px',
      mtype: 1,
      isSelected: false,
      isChildVisible: false,
      childs: []
    },
    {
      name: '达人带货',
      icon: 'icon-pc',
      code: 'shopping',
      size: '20px',
      mtype: 1,
      isSelected: false,
      isChildVisible: false,
      childs: []
    }
  ]
})

const navClkHdl = (item: any) => {
  if (item.mtype == 0) {
    states.menus.forEach((itm: any) => {
      if (item.code != itm.code) itm.isChildVisible = false
      else itm.isChildVisible = !itm.isChildVisible
    })
  } else {
    states.menus.forEach((itm: any) => {
      if (itm.mtype == 0) {
        itm.childs.forEach((chd: any) => {
          if (chd.code == item.code) chd.isSelected = true
          else chd.isSelected = false
        })
      } else if (itm.code != item.code) {
        itm.isSelected = false
      } else itm.isSelected = true
    })
    ElMessage.info(item.name)
  }
}
const expandHdl = () => {
  states.isTxt = !states.isTxt
  new animalutil(66, 211, 66)
  if (!states.isTxt) {
    states.menus.forEach((item: any) => {
      if (item.isChildVisible) states.selCode = item.code
      item.isChildVisible = false
    })
    // states.dftWdth = '66px'
    animalutil.wdth_shrink()
    nextTick(() => {
      states.dftWdth = animalutil.dft_wdth + 'px'
      console.log(1, states.dftWdth)
    })
    return
  }
  // states.dftWdth = '211px'

  animalutil.wdth_expand()
  nextTick(() => {
    states.dftWdth = animalutil.dft_wdth + 'px'
    console.log(2, states.dftWdth)
  })

  if (states.selCode) {
    states.menus.forEach((item: any) => {
      if (item.code == states.selCode) item.isChildVisible = true
    })
  }
}
</script>

<style scoped>
.pg_top {
  display: flex;
  height: 100%;
  width: 100%;
  flex: 1;
  background: #f5f5f5;
}
.pg_lft {
  display: flex;
  flex-direction: column;
}
.nav_ico {
  display: flex;
  padding: 6px 0px;
  align-items: center;
  justify-content: center;
  border-bottom: 1px solid lightgray;
}
.nav_menu {
  display: flex;
  flex: 1;
  padding: 10px 0;
  overflow-y: auto;
  overflow-x: hidden;
  flex-direction: column;
}
.menu_head {
  display: flex;
  cursor: pointer;
  align-items: center;
  justify-content: space-between;
}
.menu_item_head {
  display: flex;
  cursor: pointer;
  align-items: center;
  justify-content: flex-start;
  padding: 10px 16px 10px 19px;
}
.sub_menu_item {
  display: flex;
  cursor: pointer;
  align-items: center;
  justify-content: flex-start;
  padding: 10px 10px 10px 20px;
}
.pg_rgt {
  flex: 1;
  flex-direction: column;
  background-color: white;
}
.main_mds {
  display: flex;
  padding: 10px;
  min-height: 46px;
  max-height: 46px;
  align-items: center;
  justify-content: flex-start;
  border-bottom: 1px solid gainsboro;
}
.nav_foot {
  cursor: pointer;
  display: flex;
  padding: 11px 0px;
  align-items: center;
  justify-content: center;
  /* border-top: 1px solid lightgray; */
}

/* .sld-enter-active {
  transition: all 0.4s ease-in;
  height: auto;
  overflow: hidden;
}
.sld-leave-active {
  transition: all 0s ease-out;
  overflow: hidden;
  height: 0;
}
.sld-enter-from,
.sld-leave-to {
  transform: translateY(1px);
  opacity: 0;
  height: 0;
} */

.sld-enter-active {
  animation: slide-down 0.4s ease-in;
}
.sld-leave-active {
  animation: slide-up 0s ease-out;
}
@keyframes slide-down {
  from {
    transform: translateY(-1%);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}
@keyframes slide-up {
  from {
    transform: translateY(0);
    opacity: 1;
  }
  to {
    transform: translateY(-1%);
    opacity: 0;
  }
}
</style>
