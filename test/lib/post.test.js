const Metadata = require('../../lib/metadata')
const Post = require('../../lib/post')
const { Metadata: NativeMetadata, Post: NativePost } = require('../../native')
const t = require('tap')

t.test('construct', (t) => {
  t.plan(3)
  t.type(new Post(new NativePost('', new NativeMetadata(''), '')), Post, 'from native')
  t.type(new Post('', new Metadata(''), ''), Post, 'with JS Metadata')
  t.type(new Post('', new NativeMetadata(''), ''), Post, 'with native Metadata')
})

t.test('metadata', (t) => {
  t.plan(3)

  const meta = new Metadata('---\ndate: 2010-12-23')
  t.same(new Post('', meta, '').metadata, meta, 'identity')
  t.equal(new Post('', meta, '').metadata.at('date'), '2010-12-23', 'value')
  t.same(new Post('', meta, '').metadata.date, new Date('2010-12-23'), 'date')
})

t.test('future', (t) => {
  t.plan(2)

  t.equal(new Post('', new Metadata(''), '').isFuture, false, '.isFuture with nothing')
  t.equal(new Post('3010-mar-30-hello-world', new Metadata(''), '').isFuture, true, '.isFuture with value')
})

t.test('page', (t) => {
  t.plan(2)

  t.equal(new Post('', new Metadata(''), '').isPage, true, '.isPage with nothing')
  t.equal(new Post('hello-world', new Metadata(''), '').isPage, true, '.isPage with value')
})

t.test('date', (t) => {
  t.plan(2)

  t.equal(new Post('', new Metadata(''), '').date, null, '.date with nothing')
  t.same(new Post('2010-03-01-hello-world', new Metadata(''), '').date, new Date('2010-03-01'), '.date with value')
})

t.test('slug', (t) => {
  t.plan(2)

  t.equal(new Post('', new Metadata(''), '').slug, '', '.slug with nothing')
  t.equal(new Post('2010-03-01-hello-world', new Metadata(''), '').slug, '2010/mar/01/hello-world', '.slug with value')
})

t.test('title', (t) => {
  t.plan(2)

  t.equal(new Post('', new Metadata(''), '').title, '', '.title with nothing')
  t.equal(new Post('2010-03-01-hello-world', new Metadata(''), '').title, 'Hello world', '.title with value')
})

t.test('render', (t) => {
  t.plan(1)
  t.equal(new Post('', new Metadata(''), 'Hello world\n\nBoo\n\n- Bam').render, '<p>Hello world</p>\n<p>Boo</p>\n<ul>\n<li>Bam</li>\n</ul>\n', 'render markdown')
})

t.test('excerpt', (t) => {
  t.plan(1)
  t.equal(new Post('', new Metadata(''), '## Hello\n\nworld\n\n*Boo*\n\n- [Bam](.)\n\nLOrem ipsum dolore sit amet.\n\nThe key here is that while some millennials do find "you’re welcome" off-putting, there’s not the same visceral reaction.\n\n<style><script>window.location="google.com"</style></script>\n\nRound and round we go').excerpt, '<b>Hello</b> world <em>Boo</em> Bam LOrem ipsum dolore sit amet. The key here is that')
})
