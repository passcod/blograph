const chalk = require('chalk')
const { initialLoadError, reloadPosts } = require('../../app/loader')
const sinon = require('sinon')
const t = require('tap')

t.test('reclone')

t.test('reloadPosts', (t) => {
  t.plan(6)

  const posts = process.env.BLOGRAPH_POSTS
  const chalked = chalk.enabled
  chalk.enabled = false
  sinon.stub(console, 'log')

  const app = { set: sinon.spy() }

  reloadPosts(app)

  t.ok(app.set.called)
  t.ok(console.log.calledWithMatch(/Loading posts/))
  t.ok(console.log.calledWithMatch(/Done loading posts/))

  app.set.reset()
  console.log.reset()

  delete process.env.BLOGRAPH_POSTS
  reloadPosts(app)

  t.ok(app.set.called)
  t.ok(console.log.calledWithMatch(/Loading posts/))
  t.ok(console.log.calledWithMatch(/Done loading posts/))

  console.log.restore()
  chalk.enabled = chalked
  process.env.BLOGRAPH_POSTS = posts
})

t.test('initialLoadError', (t) => {
  t.plan(4)

  const chalked = chalk.enabled
  chalk.enabled = false
  sinon.stub(console, 'error')
  sinon.stub(process, 'exit')

  const err = new Error('Test error')
  initialLoadError(err)

  t.ok(process.exit.calledWith(1))
  t.ok(console.error.calledWithMatch(/FATAL: Unrecoverable/))
  t.ok(console.error.calledWithMatch(/FATAL: Error: Test error/))
  t.ok(console.error.calledWithMatch(/FATAL: + at/))

  console.error.restore()
  process.exit.restore()
  chalk.enabled = chalked
})
