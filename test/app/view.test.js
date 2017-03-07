const view = require('../../app/view')
const t = require('tap')

t.test('view')

t.test('htmlstrip', (t) => {
  t.plan(1)
  t.equal(view.htmlstrip('Hello <b>world</b>'), 'Hello world')
})
