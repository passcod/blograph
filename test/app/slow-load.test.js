const request = require('supertest')
const t = require('tap')

function sleep (ms) {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

t.test('fails on bad load', async (t) => {
  process.env.SLOW_LOAD = true

  const app = require('../../app')
  await request(app).get('/healthz').expect(503)

  await sleep(200)

  await request(app).get('/healthz').expect(204)
})
