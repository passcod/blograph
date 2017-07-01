const { load } = require('../../native')
const t = require('tap')

t.test((t) => {
  t.plan(2)

  const all = load(process.env.BLOGRAPH_POSTS)
  t.equal(all.toArray().length, 5, 'there are some files here')
  t.equal(all.toArray()[0].slug(), '2013/aug/10/grow', 'that happen to have a name')
})
