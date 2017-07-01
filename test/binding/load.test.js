const { load } = require('../../native')
const path = require('path')
const t = require('tap')

t.test((t) => {
  t.plan(2)

  const all = load(path.resolve('./test/posts/'))
  t.equal(all.toArray().length, 4, 'there are some files here')
  t.equal(all.toArray()[0].slug(), '2013/aug/10/grow', 'that happen to have a name')
})
