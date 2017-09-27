const chalk = require('chalk')
const { initialLoadError, reclone, reloadPosts } = require('../../app/loader')
const sinon = require('sinon')
const t = require('tap')

t.test('reclone', async (t) => {
  const posts = process.env.BLOGRAPH_POSTS
  const repo = process.env.BLOGRAPH_REPO
  const chalked = chalk.enabled
  chalk.enabled = false
  sinon.stub(console, 'error')
  sinon.stub(console, 'log')
  sinon.stub(process, 'exit').callsFake(() => Promise.reject(new Error()))

  let resolved
  await reclone().then(() => { resolved = true })

  t.ok(console.error.calledWithMatch(/BLOGRAPH_POSTS is set/))
  t.notOk(process.exit.called)
  t.ok(resolved)

  delete process.env.BLOGRAPH_REPO
  delete process.env.BLOGRAPH_POSTS

  let rejected
  await reclone().catch(() => { rejected = true })

  t.ok(rejected)

  resolved = false
  process.env.BLOGRAPH_REPO = repo || './test/posts'
  await reclone().then(() => { resolved = true })

  t.ok(console.log.calledWithMatch(new RegExp(`Cloning ${process.env.BLOGRAPH_REPO}`)))
  t.ok(console.log.calledWithMatch(/Done cloning/))
  t.ok(resolved)

  console.error.restore()
  console.log.restore()
  process.exit.restore()
  chalk.enabled = chalked
  process.env.BLOGRAPH_POSTS = posts
  process.env.BLOGRAPH_REPO = repo
})

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
