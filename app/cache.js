const cacheControl = require('express-cache-controller')
const ms = require('ms')

module.exports = function (ttl = '5m') {
  return cacheControl({ maxAge: ms(ttl) })
}
