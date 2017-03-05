const { List, Metadata, Post } = require('../native')
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
