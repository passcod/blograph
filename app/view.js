const fs = require('fs')
const moment = require('moment')
const plur = require('plur')
const sanitize = require('sanitize-html')
const { version } = require('../package.json')
const words = require('number-to-words').toWords

function htmlstrip (html) {
  return sanitize(html, {
    allowedTags: [],
    allowedAttributes: []
  })
}

function asset (file) {
  return `/assets/${version}/${file}`
}

const inlines = {}
function inline (file) {
  if (!inlines[file]) {
    inlines[file] = fs.readFileSync(`./assets/${file}`)
  }

  return inlines[file]
}

function view (req, res, next) {
  res.view = (partial, locals = {}) => res.render('layout', {
    asset,
    htmlstrip,
    inline,
    locals,
    moment,
    partial,
    plur,
    words
  })

  next()
}

module.exports = view
module.exports.htmlstrip = htmlstrip
