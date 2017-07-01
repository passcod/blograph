const chalk = require('chalk')
const formatter = require('format-number')
const morgan = require('morgan')
const ms = require('ms')

function elapsed (responseTime) {
  const time = ms(responseTime)
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

  return elapsed.join('')
}

function statusColour (code) {
  return code >= 400
    ? 'red'
    : (code >= 200 && code < 300)
      ? 'green'
      : 'blue'
}

module.exports = morgan((tokens, req, res) => {
  const status = +tokens.status(req, res)

  return [
    chalk.cyan(elapsed(+tokens['response-time'](req, res))),
    chalk.magenta(tokens.method(req, res)),
    chalk.bold('→'),
    chalk[statusColour(status)](status),
    tokens.url(req, res)
  ].join(' ')
})

// Exposed for testing
module.exports.elapsed = elapsed
module.exports.statusColour = statusColour
