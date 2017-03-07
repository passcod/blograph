const blograph = require('../../lib')
const t = require('tap')

t.test('members', (t) => {
  t.plan(4)

  t.type(blograph.List, 'function', 'List')
  t.type(blograph.load, 'function', '.load()')
  t.type(blograph.Metadata, 'function', 'Metadata')
  t.type(blograph.Post, 'function', 'Post')
})

t.test('.load()', (t) => {
  t.plan(2)

  const posts = blograph.load('./test/posts')
  t.type(posts, blograph.List, 'returns a JS List')
  t.equal(posts.length, 3, 'loads some posts')
})
