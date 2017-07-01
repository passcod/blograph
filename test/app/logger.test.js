const { elapsed, statusColour } = require('../../app/logger')
const t = require('tap')

t.test('elapsed', (t) => {
  t.plan(5)

  t.equal('  0.000ms', elapsed(0))
  t.equal('  1.000 s', elapsed(1000))
  t.equal('  4.000 m', elapsed(238912))
  t.equal('427.000ms', elapsed(427))
  t.equal(' 29.000 s', elapsed(28971))
})

t.test('statusColour', (t) => {
  t.plan(16)

  t.equal(statusColour(0), 'blue')
  t.equal(statusColour(100), 'blue')
  t.equal(statusColour(102), 'blue')
  t.equal(statusColour(200), 'green')
  t.equal(statusColour(204), 'green')
  t.equal(statusColour(301), 'blue')
  t.equal(statusColour(302), 'blue')
  t.equal(statusColour(307), 'blue')
  t.equal(statusColour(400), 'red')
  t.equal(statusColour(403), 'red')
  t.equal(statusColour(404), 'red')
  t.equal(statusColour(429), 'red')
  t.equal(statusColour(500), 'red')
  t.equal(statusColour(501), 'red')
  t.equal(statusColour(600), 'red')
  t.equal(statusColour(1273), 'red')
})
