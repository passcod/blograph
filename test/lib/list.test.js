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

t.test('reverse', (t) => {
  t.plan(3)

  const posts = new List([
    new Post('world', new Metadata(''), ''),
    new Post('jolly', new Metadata(''), ''),
    new Post('hello', new Metadata(''), '')
  ]).reverse

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

t.test('findBySlug', (t) => {
  t.plan(3)

  const posts = new List([
    new Post('hello', new Metadata(''), ''),
    new Post('jolly', new Metadata(''), ''),
    new Post('world', new Metadata(''), '')
  ])

  t.equal(posts.findBySlug('hello').slug, 'hello')
  t.notOk(posts.findBySlug('there'))
  t.equal(posts.findBySlug('world').slug, 'world')
})

t.test('sortByDate', (t) => {
  t.plan(3)

  const posts = new List([
    new Post('metadated', new Metadata('---\ndate: 2015-01-01'), ''),
    new Post('2016-jan-01-filedated', new Metadata(''), ''),
    new Post('2017-jun-30-bothdated', new Metadata('---\ndate: 2017-06-30T20:37:50+12:00'), ''),
    new Post('undated', new Metadata(''), ''),
    new Post('nopedated', new Metadata(''), '')
  ]).sortByDate()

  t.same(posts.map(({ post }) => post.slug), ['undated', 'nopedated', '2015/jan/01/metadated', '2016/jan/01/filedated', '2017/jun/30/bothdated'])
  t.same(posts.map(({ prev }) => prev && prev.slug), [null, 'undated', 'nopedated', '2015/jan/01/metadated', '2016/jan/01/filedated'])
  t.same(posts.map(({ next }) => next && next.slug), ['nopedated', '2015/jan/01/metadated', '2016/jan/01/filedated', '2017/jun/30/bothdated', null])
})

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

t.test('tags', (t) => {
  t.plan(6)

  const noPosts = new List([]).tags

  const noTags = new List([
    new Post('none', new Metadata(''), ''),
    new Post('empty', new Metadata('---\ntags:'), '')
  ]).tags

  const someTags = new List([
    new Post('none', new Metadata(''), ''),
    new Post('one', new Metadata('---\ntags:\n  - one'), ''),
    new Post('some', new Metadata('---\ntags:\n  - one\n  - two\n  - three'), ''),
    new Post('dupe', new Metadata('---\ntags:\n  - three\n  - two'), ''),
    new Post('empty', new Metadata('---\ntags:'), '')
  ]).tags

  t.type(noPosts, Set)
  t.type(noTags, Set)
  t.type(someTags, Set)

  t.equal(noPosts.size, 0, 'with no posts')
  t.equal(noTags.size, 0, 'with no tags')
  t.equal(someTags.size, 3, 'with some tags')
})

t.test('parentsOf', (t) => {
  t.plan(9)

  const postA = new Post('a', new Metadata(''), '')
  const postB = new Post('b', new Metadata('---\nparent: a'), '')
  const postC = new Post('c', new Metadata('---\nparents:'), '')
  const postD = new Post('d', new Metadata('---\nparents:\n  - a'), '')
  const postE = new Post('e', new Metadata('---\nparents:\n  - c\n  - d'), '')

  const posts = new List([ postA, postB, postC, postD, postE ])

  t.equal(posts.parentsOf(postA).length, 0)
  t.equal(posts.parentsOf(postB).length, 1)
  t.equal(posts.parentsOf(postB).toArray()[0].slug, 'a')
  t.equal(posts.parentsOf(postC).length, 0)
  t.equal(posts.parentsOf(postD).length, 1)
  t.equal(posts.parentsOf(postD).toArray()[0].slug, 'a')
  t.equal(posts.parentsOf(postE).length, 2)
  t.equal(posts.parentsOf(postE).toArray()[0].slug, 'c')
  t.equal(posts.parentsOf(postE).toArray()[1].slug, 'd')
})

t.test('childrenOf', (t) => {
  t.plan(9)

  const postA = new Post('a', new Metadata(''), '')
  const postB = new Post('b', new Metadata('---\nparent: a'), '')
  const postC = new Post('c', new Metadata('---\nparents:'), '')
  const postD = new Post('d', new Metadata('---\nparents:\n  - a'), '')
  const postE = new Post('e', new Metadata('---\nparents:\n  - c\n  - d'), '')

  const posts = new List([ postA, postB, postC, postD, postE ])

  t.equal(posts.childrenOf(postA).length, 2)
  t.equal(posts.childrenOf(postA).toArray()[0].slug, 'b')
  t.equal(posts.childrenOf(postA).toArray()[1].slug, 'd')
  t.equal(posts.childrenOf(postB).length, 0)
  t.equal(posts.childrenOf(postC).length, 1)
  t.equal(posts.childrenOf(postC).toArray()[0].slug, 'e')
  t.equal(posts.childrenOf(postD).length, 1)
  t.equal(posts.childrenOf(postD).toArray()[0].slug, 'e')
  t.equal(posts.childrenOf(postE).length, 0)
})
