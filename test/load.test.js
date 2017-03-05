const { load, List, Metadata, Post } = require('../native')
const path = require('path')
const t = require('tap')

t.test((t) => {
  t.plan(1)

  const all = load(path.resolve('./test/posts/'))
  t.equal(all.toArray().length, 3, 'there are some files here')
})
