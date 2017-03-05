const List = require('../../lib/blograph/list')
const Metadata = require('../../lib/blograph/metadata')
const Post = require('../../lib/blograph/post')
const { List: NativeList, Metadata: NativeMetadata, Post: NativePost } = require('../../native')
const t = require('tap')

t.test('construct', (t) => {
  t.plan(4)

  t.type(new List([]), List, 'type')
  t.type(new List([new Post('', new Metadata(''), '')]), List, 'with JS Post')
  t.type(new List([new NativePost('', new NativeMetadata(''), '')]), List, 'with native Post')
  t.type(new List(new NativeList([new NativePost('', new NativeMetadata(''), '')])), List, 'with native List')
})

t.test('toArray', (t) => {
  t.plan(2)

  const posts = [new Post('', new Metadata(''), '')]
  t.same(new List(posts).toArray(), posts, '.toArray()')
  t.same(new List([]).toArray(), [], '.toArray() with empty list')
})

t.test('iter', (t) => {
  t.plan(7)

  const posts = new List([
    new Post('hello', new Metadata(''), ''),
    new Post('jolly', new Metadata(''), ''),
    new Post('world', new Metadata(''), '')
  ])

  let i = 0
  for (const { post, prev, next } of posts) {
    t.comment(post.slug)
    if (i === 0) {
      t.notOk(prev, 'first iteration has no prev')
      t.type(next, 'Post', 'first iteration has a next')
    } else if (i === 2) {
      t.notOk(next, 'last iteration has no next')
      t.type(prev, 'Post', 'last iteration has a prev')
    }

    t.type(post, 'Post', 'all iterations have a post')

    i += 1
  }
})

t.test('forEach', (t) => {
  t.plan(7)

  const posts = new List([
    new Post('hello', new Metadata(''), ''),
    new Post('jolly', new Metadata(''), ''),
    new Post('world', new Metadata(''), '')
  ])

  posts.forEach(({ post, prev, next }, i) => {
    t.comment(post.slug)
    if (i === 0) {
      t.notOk(prev, 'first iteration has no prev')
      t.type(next, 'Post', 'first iteration has a next')
    } else if (i === 2) {
      t.notOk(next, 'last iteration has no next')
      t.type(prev, 'Post', 'last iteration has a prev')
    }

    t.type(post, 'Post', 'all iterations have a post')
  })
})

t.test('map', (t) => {
  t.plan(3)

  const posts = new List([
    new Post('hello', new Metadata(''), ''),
    new Post('jolly', new Metadata(''), ''),
    new Post('world', new Metadata(''), '')
  ])

  t.same(posts.map(({ post }) => post.slug), ['hello', 'jolly', 'world'])
  t.same(posts.map(({ prev }) => prev && prev.slug), [null, 'hello', 'jolly'])
  t.same(posts.map(({ next }) => next && next.slug), ['jolly', 'world', null])
})

t.test('filter', (t) => {
  t.plan(2)

  const posts = new List([
    new Post('hello', new Metadata(''), ''),
    new Post('jolly', new Metadata(''), ''),
    new Post('world', new Metadata(''), '')
  ])

  const filtered = posts.filter(({ post }) => post.slug !== 'jolly')
  t.same(filtered.map(({ post }) => post.slug), ['hello', 'world'])
  t.type(filtered, List, '.filter() returns a List')
})
