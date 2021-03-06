const app = require('../../app')
const request = require('supertest')
const t = require('tap')
const { version } = require('../../package.json')

t.test('/healthz', (t) => request(app)
  .get('/healthz')
  .expect(204)
)

t.test('/version', (t) => request(app)
  .get('/version')
  .expect('Content-Type', /text\/plain/)
  .expect(200)
  .expect(version)
)

t.test('/assets/$version/style.css', (t) => request(app)
  .get(`/assets/${version}/style.css`)
  .expect('Content-Type', /css/)
  .expect(200)
  .expect(/\*\s*\{\s*box-sizing:/)
)

t.test('/hook/reload/posts', (t) => request(app)
  .post('/hook/reload/posts')
  .expect(204)
)

t.test('/feed', (t) => request(app)
  .get('/feed')
  .expect('Content-Type', /rss/)
  .expect(200)
  .expect(/Feed of the front page/)
  .expect(/my birthday today/)
  .expect(/A verb./)
  .expect(/Install mosh/)
)

t.test('/', (t) => request(app)
  .get('/')
  .expect('Content-Type', /html/)
  .expect(200)
  .expect(/article class/)
)

t.test('/tag/:tag', (t) => request(app)
  .get('/tag/linux')
  .expect('Content-Type', /html/)
  .expect(200)
  .expect(/Tag: linux/)
  .expect(/Ever encountered this/)
)

t.test('/2015/jul/16/tonight', (t) => request(app)
  .get('/2015/jul/16/tonight')
  .expect('Content-Type', /html/)
  .expect(200)
  .expect(/<title>\s*Tonight/)
)

t.test('/2015/may/04/today', (t) => request(app)
  .get('/2015/may/04/today')
  .expect('Content-Type', /html/)
  .expect(200)
  .expect(/#StarWarsDay/)
  .expect(/<title>\s*Today/)
)

t.test('/3012/jul/16/tomorrow', (t) => request(app)
  .get('/3012/jul/16/tomorrow')
  .expect('Content-Type', /html/)
  .expect(200)
  .expect(/Tomorrow/)
)

t.test('/not-found', (t) => request(app)
  .get('/not-found')
  .expect('Content-Type', /html/)
  .expect(404)
  .expect(/<title>\s*Not Found/)
)

t.test('/2010/jan/10/not-found', (t) => request(app)
  .get('/2010/jan/10/not-found')
  .expect('Content-Type', /html/)
  .expect(404)
  .expect(/<title>\s*Not Found/)
)
