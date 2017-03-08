const chalk = require('chalk')
const formatter = require('format-number')
const morgan = require('morgan')
const ms = require('ms')

module.exports = morgan((tokens, req, res) => {
  const time = ms(+tokens['response-time'](req, res))
  const amount = parseFloat(time)
  const unit = /[\d.]+(.+)$/.exec(time)[1]

  const format = formatter({
    suffix: unit.length < 2 ? ` ${unit}` : unit,
    noSeparator: true,
    padLeft: 3,
    padRight: 3,
    round: 3
  })

  let i = 0
  let elapsed = format(amount).split('')
  while (elapsed[i] === '0' && elapsed[i + 1] !== '.') {
    elapsed[i] = ' '
    i += 1
  }

  elapsed = elapsed.join('')

  const status = +tokens.status(req, res)
  const colour = status > 400
    ? 'red'
    : (status >= 200 && status < 300)
      ? 'green'
      : 'blue'

  return [
    chalk.cyan(elapsed),
    chalk.magenta(tokens.method(req, res)),
    chalk.bold('→'),
    chalk[colour](status),
    tokens.url(req, res)
  ].join(' ')
})
