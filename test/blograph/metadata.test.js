const Metadata = require('../../lib/blograph/metadata')
const { Metadata: NativeMetadata } = require('../../native')
const t = require('tap')

t.test('parsing and access', (t) => {
  t.plan(9)

  t.type(new Metadata(''), Metadata, 'new Metadata() makes a new class')
  t.type(new Metadata(new NativeMetadata('')), Metadata, 'can be constructed with a native type')

  t.equal(new Metadata('---\nfoo: 123.456').at('foo'), 123.456, '.at() with float')
  t.same(new Metadata('---\nfoo: [1,2,3]').at('foo'), [1, 2, 3], '.at() with array')
  t.same(new Metadata('---\nfoo:\n  bar: 2').at('foo'), {bar: 2}, '.at() with hash')
  t.equal(new Metadata('---\nfoo:\n bar:\n  baz: qux').at('foo.bar.baz'), 'qux', 'dotted .at()')

  t.equal(new Metadata('---\nfoo: bar').string('foo'), 'bar', '.string() with string')
  t.equal(new Metadata('---\nfoo: true').bool('foo'), true, '.bool() with bool')
  t.equal(new Metadata('---\nfoo: 123').int('foo'), 123, '.int() with int')
})

t.test('specific fields', (t) => {
  t.plan(18)

  t.equal(new Metadata('---\npage: true').page, true, '.page with true')
  t.equal(new Metadata('---\npage: false').page, false, '.page with false')
  t.equal(new Metadata('').page, false, '.page default')
  t.equal(new Metadata('---\ndate: 2010-11-12').page, false, '.page with date')

  t.equal(new Metadata('').date, null, '.date default')
  t.equal(new Metadata('---\ndate: invalid').date, null, '.date invalid')
  t.same(new Metadata('---\ndate: 2010-01-23').date, new Date('2010-01-23'), '.date is a JS Date')
  t.same(new Metadata('---\ndate: 2010-01-23T12:34:56+13:00').date, new Date('2010-01-22T23:34:56Z'), '.date with timezone')

  t.same(new Metadata('').parents, [], '.parents with nothing')
  t.same(new Metadata('---\nparents:\n - foo\n - bar').parents, ['foo', 'bar'], '.parents with multiple in array')

  t.equal(new Metadata('').author, null, '.author with nothing')
  t.equal(new Metadata('---\nauthor: me').author, 'me', '.author with value')

  t.same(new Metadata('').tags, [], '.tags with nothing')
  t.same(new Metadata('---\ntags:\n - foo\n - bar').tags, ['foo', 'bar'], '.tags with multiple in array')

  t.equal(new Metadata('').kind, null, '.kind with nothing')
  t.equal(new Metadata('---\ntags:\n - foo\n - bar').kind, 'foo', '.kind with tags')

  t.equal(new Metadata('').title, null, '.title with nothing')
  t.equal(new Metadata('---\ntitle: Hi').title, 'Hi', '.title with value')
})
