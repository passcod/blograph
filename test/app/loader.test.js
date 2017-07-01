const chalk = require('chalk')
const { initialLoadError } = require('../../app/loader')
const sinon = require('sinon')
const t = require('tap')

t.test('reclone')
t.test('reloadPosts')

t.test('initialLoadError', (t) => {
  t.plan(4)
  const chalked = !!chalk.enabled
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
