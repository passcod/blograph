const chalk = require('chalk')
const sinon = require('sinon')
const t = require('tap')

t.test('fails on bad load', (t) => {
  t.plan(4)

  chalk.enabled = false
  sinon.stub(console, 'error')
  sinon.stub(process, 'exit')

  delete process.env.BLOGRAPH_REPO
  delete process.env.BLOGRAPH_POSTS

  require('../../app')

  setTimeout(() => {
    t.ok(process.exit.calledWith(1))
    t.ok(console.error.calledWithMatch(/FATAL: Unrecoverable/))
    t.ok(console.error.calledWithMatch(/FATAL: Error: Neither BLOGRAPH_REPO/))
    t.ok(console.error.calledWithMatch(/FATAL: + at/))
  }, 200)
})
