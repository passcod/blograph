'use strict'

function cache (context, name, value) {
  Object.defineProperty(context, name, { value })
  return value
}

module.exports = cache
