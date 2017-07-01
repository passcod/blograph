const view = require('../../app/view')
const sinon = require('sinon')
const t = require('tap')

t.test('view', (t) => {
  t.plan(4)

  const res = { render: sinon.spy() }
  const next = sinon.spy()
  view({}, res, next)
  res.view('foobar')

  t.type(res.view, Function)
  t.equal(res.view.length, 1, 'only one required argument')
  t.ok(res.render.called)
  t.ok(next.called)
})

t.test('htmlstrip', (t) => {
  t.plan(1)
  t.equal(view.htmlstrip('Hello <b>world</b>'), 'Hello world')
})
