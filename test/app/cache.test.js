const cache = require('../../app/cache')
const t = require('tap')

t.test('cache middleware', (t) => {
  t.plan(2)
  t.type(cache('10m'), Function)
  t.type(cache(), Function)
})
