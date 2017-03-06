const { List, Metadata, Post } = require('../../native')
const t = require('tap')

t.test('construct', (t) => {
  t.plan(3)

  t.type(new List([]), 'object', 'type')
  t.throws(() => new List([123]), 'bad argument type')
  t.type(new List([new Post('', new Metadata(''), '')]), 'object', 'with Post')
})

t.test('toArray', (t) => {
  t.plan(2)

  const posts = [new Post('', new Metadata(''), '')]
  t.same(new List(posts).toArray(), posts, '.toArray()')
  t.same(new List([]).toArray(), [], '.toArray() with empty list')
})

t.test('length', (t) => {
  t.plan(2)

  const posts = [new Post('', new Metadata(''), '')]
  t.same(new List(posts).length(), 1, '.length()')
  t.same(new List([]).length(), 0, '.length() with empty list')
})

t.test('iter', (t) => {
  t.plan(7)

  const posts = new List([
    new Post('hello', new Metadata(''), ''),
    new Post('jolly', new Metadata(''), ''),
    new Post('world', new Metadata(''), '')
  ])

  let i = 0
  posts.iter((post, prev, next) => {
    if (i === 0) {
      t.notOk(prev, 'first iteration has no prev')
      t.type(next, 'Post', 'first iteration has a next')
    } else if (i === 2) {
      t.notOk(next, 'last iteration has no next')
      t.type(prev, 'Post', 'last iteration has a prev')
    }

    t.type(post, 'Post', 'all iterations have a post')

    i += 1
  })
})
