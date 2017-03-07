const app = require('../../app')
const request = require('supertest')
const t = require('tap')

t.test('post not found', (t) => request(app)
  .get('/2017/jan/01/post-does-not-exist')
  .expect('Content-Type', /html/)
  .expect(404)
)
