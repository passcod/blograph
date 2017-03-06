const bytes = require('bytes')
const chalk = require('chalk')
const compression = require('compression')
const express = require('express')
const helmet = require('helmet')
const moment = require('moment')
const morgan = require('morgan')
const ms = require('ms')
const { load } = require('../lib/blograph')

const app = module.exports = express()
app.set('view engine', 'ejs')
app.set('posts', load('./posts'))

app.use(morgan((tokens, req, res) => [
  [
    chalk.bold('←'),
    chalk.magenta(tokens.method(req, res)),
    tokens.url(req, res)
  ].join(' '),
  [
    chalk.bold('→'),
    chalk.magenta(tokens.status(req, res)),
    '(' + chalk.cyan(bytes.format(+tokens.res(req, res, 'content-length'))) + ')',
    '(' + chalk.blue(ms(+tokens['response-time'](req, res))) + ')'
  ].join(' ')
].join('\n')))

app.use(compression())
app.use(helmet())
app.use(express.static('./public'))
app.use((req, res, next) => {
  const globals = { moment }
  res.view = (partial, locals = {}) => res.render('layout', Object.assign(
    { partial },
    globals,
    { locals: Object.assign({}, globals, locals) }
  ))

  next()
})

app.get('/', (req, res) =>
  res.view('index', { posts: req.app.get('posts') })
)
