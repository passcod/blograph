const { Metadata, Post } = require('../native')
const t = require('tap')

t.test('construct', (t) => {
  t.plan(1)
  t.type(new Post('', new Metadata(''), ''), 'object')
})

t.test('metadata', (t) => {
  t.plan(3)

  const meta = new Metadata('---\ndate: 2010-12-23')
  t.same(new Post('', meta, '').metadata(), meta, 'identity')
  t.equal(new Post('', meta, '').metadata().at('date'), '2010-12-23', 'value')
  t.equal(new Post('', meta, '').metadata().date(), '2010-12-23T00:00:00+00:00', 'date')
})

t.test('future', (t) => {
  t.plan(4)

  t.equal(new Post('', new Metadata(''), '').isFuture(), false, '.isFuture() with nothing')
  t.equal(new Post('2010-mar-30-hello-world', new Metadata(''), '').isFuture(), false, '.isFuture() with path')
  t.equal(new Post('3010-mar-30-hello-world', new Metadata(''), '').isFuture(), true, '.isFuture() with future path')
  t.equal(new Post('2010-mar-30-hello-world', new Metadata('---\ndate: 3010-02-01'), '').isFuture(), true, '.isFuture() with path and meta')
})

t.test('page', (t) => {
  t.plan(6)

  t.equal(new Post('', new Metadata(''), '').isPage(), true, '.isPage() with nothing')
  t.equal(new Post('hello-world', new Metadata(''), '').isPage(), true, '.isPage() with path')
  t.equal(new Post('2011-12-13-hello-world', new Metadata(''), '').isPage(), false, '.isPage() with dated path')
  t.equal(new Post('2011-12-13-hello-world', new Metadata('---\npage: true'), '').isPage(), true, '.isPage() with dated path and metadata page')
  t.equal(new Post('hello-world', new Metadata('---\ndate: 2003-04-05'), '').isPage(), false, '.isPage() with metadata date')
  t.equal(new Post('hello-world', new Metadata('---\ndate: 2003-04-05\npage: true'), '').isPage(), true, '.isPage() with metadata date and page')
})

t.test('date', (t) => {
  t.plan(4)

  t.equal(new Post('', new Metadata(''), '').date(), null, '.date() with nothing')
  t.equal(new Post('2010-03-01-hello-world', new Metadata(''), '').date(), '2010-03-01T00:00:00+00:00', '.date() with dated path')
  t.equal(new Post('hello-world', new Metadata('---\ndate: 2010-03-01'), '').date(), '2010-03-01T00:00:00+00:00', '.date() with metadata date')
  t.equal(new Post('2010-03-01-hello-world', new Metadata('---\ndate: 2016-07-23'), '').date(), '2016-07-23T00:00:00+00:00', '.date() with dated path and metadata date')
})

t.test('slug', (t) => {
  t.plan(5)

  t.equal(new Post('', new Metadata(''), '').slug(), '', '.slug() with nothing')
  t.equal(new Post('2010-03-01-hello-world', new Metadata(''), '').slug(), '2010/mar/01/hello-world', '.slug() with dated path')
  t.equal(new Post('hello-world', new Metadata(''), '').slug(), 'hello-world', '.slug() with non-dated path')
  t.equal(new Post('2010-03-01-hello-world', new Metadata('---\ntitle: Foo bar'), '').slug(), '2010/mar/01/hello-world', '.slug() with metadata title')
  t.equal(new Post('2010-03-01-hello-world', new Metadata('---\ndate: 2012-01-28'), '').slug(), '2012/jan/28/hello-world', '.slug() with metadata date')
})

t.test('title', (t) => {
  t.plan(3)

  t.equal(new Post('', new Metadata(''), '').title(), '', '.title() with nothing')
  t.equal(new Post('2010-03-01-hello-world', new Metadata(''), '').title(), 'Hello world', '.title() with path')
  t.equal(new Post('2010-03-01-hello-world', new Metadata('---\ntitle: Foo bar'), '').title(), 'Foo bar', '.title() with metadata title')
})

t.test('render', (t) => {
  t.plan(1)
  t.equal(new Post('', new Metadata(''), 'Hello world\n\nBoo\n\n- Bam').render(), '<p>Hello world</p>\n<p>Boo</p>\n<ul>\n<li>Bam</li>\n</ul>\n', 'render markdown')
})
