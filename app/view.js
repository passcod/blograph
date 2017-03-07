const moment = require('moment')
const plur = require('plur')
const sanitize = require('sanitize-html')
const words = require('number-to-words').toWords

function htmlstrip (html) {
  return sanitize(html, {
    allowedTags: [],
    allowedAttributes: []
  })
}

module.exports = (req, res, next) => {
  res.view = (partial, locals = {}) => res.render('layout', {
    htmlstrip,
    locals,
    moment,
    partial,
    plur,
    words
  })
  next()
}
