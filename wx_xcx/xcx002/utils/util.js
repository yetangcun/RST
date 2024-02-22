const utils = require('md5.js')

const formatTime = date => {
  const year = date.getFullYear()
  const month = date.getMonth() + 1
  const day = date.getDate()
  const hour = date.getHours()
  const minute = date.getMinutes()
  const second = date.getSeconds()
  return `${[year, month, day].map(formatNumber).join('/')} ${[hour, minute, second].map(formatNumber).join(':')}`
}

const formatNumber = n => {
  n = n.toString()
  return n[1] ? n : `0${n}`
}

const base64Encode = sur => {
  let base64 = new utils.Base64();
  return base64.encode(sur);
}

const md5 = sur => {
  let  md5Str = utils.md5(sur)
  return md5Str
}

module.exports = {
  md5,
  formatTime,
  base64Encode
}
