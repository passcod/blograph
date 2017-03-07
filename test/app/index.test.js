const app = require('../../app')
const request = require('supertest')
const t = require('tap')

t.test('/feed')
t.test('/tag/:tag')
t.test('/hooks/reload/posts')

t.test('always responds', (t) => request(app)
  .get('/')
  .expect('Content-Type', /html/)
  .expect(200)
  .expect(/article class/)
)
