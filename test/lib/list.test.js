const List = require('../../lib/list')
const Metadata = require('../../lib/metadata')
const Post = require('../../lib/post')
const { List: NativeList, Metadata: NativeMetadata, Post: NativePost } = require('../../native')
const t = require('tap')

t.test('construct', (t) => {
  t.plan(5)

  t.type(new List([]), List, 'type')
  t.throws(() => new List([123, 'foo', false]), 'with wrong Post types')
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

t.test('length', (t) => {
  t.plan(2)

  const posts = [new Post('', new Metadata(''), '')]
  t.same(new List(posts).length, 1, '.length')
  t.same(new List([]).length, 0, '.length with empty list')
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
  t.plan(3)

  const posts = new List([
    new Post('hello', new Metadata(''), ''),
    new Post('jolly', new Metadata(''), ''),
    new Post('world', new Metadata(''), '')
  ])

  // Sanity check
  t.same(posts.map(({ post }) => post.slug), ['hello', 'jolly', 'world'])

  const filtered = posts.filter(({ post }) => post.slug !== 'jolly')
  t.same(filtered.map(({ post }) => post.slug), ['hello', 'world'])
  t.type(filtered, List, '.filter() returns a List')
})

t.test('findBySlug')
t.test('sortByDate')

t.test('includes', (t) => {
  t.plan(3)

  const post0 = new Post('hello', new Metadata(''), '')
  const post1 = new Post('jolly', new Metadata(''), '')
  const post2 = new Post('world', new Metadata(''), '')

  const posts = new List([ post0, post1, post2 ])
  t.equal(posts.includes(post1), true, 'with present post')

  // Sanity check
  t.same(posts.map(({ post }) => post.slug), ['hello', 'jolly', 'world'])

  const filtered = posts.filter(({ post }) => post.slug !== 'jolly')
  t.equal(filtered.includes(post1), false, 'with missing post')
})

t.test('tags')
t.test('parentsOf')
t.test('childrenOf')
