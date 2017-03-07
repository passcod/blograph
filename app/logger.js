const bytes = require('bytes')
const chalk = require('chalk')
const morgan = require('morgan')
const ms = require('ms')

module.exports = morgan((tokens, req, res) => [
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
].join('\n'))
