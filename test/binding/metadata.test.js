const { Metadata } = require('../../native')
const t = require('tap')

t.test('parsing and access', (t) => {
  t.plan(35)

  t.type(new Metadata(''), 'Metadata', 'new Metadata() makes a new type')

  t.equal(JSON.stringify(new Metadata('---\nfoo: bar')), '{"foo":"bar"}', 'serialise string')
  t.equal(JSON.stringify(new Metadata('---\nfoo: 123')), '{"foo":123}', 'serialise int')
  t.equal(JSON.stringify(new Metadata('---\nfoo: 123.456')), '{"foo":123.456}', 'serialise float')
  t.equal(JSON.stringify(new Metadata('---\nfoo: true')), '{"foo":true}', 'serialise bool')
  t.equal(JSON.stringify(new Metadata('---\nfoo: [1,2,3]')), '{"foo":[1,2,3]}', 'serialise array')
  t.equal(JSON.stringify(new Metadata('---\nfoo:\n  bar: 2')), '{"foo":{"bar":2}}', 'serialise hash')

  t.equal(new Metadata('---\nfoo: bar').at('foo'), 'bar', '.at() with string')
  t.equal(new Metadata('---\nfoo: 123').at('foo'), 123, '.at() with int')
  t.equal(new Metadata('---\nfoo: 123.456').at('foo'), 123.456, '.at() with float')
  t.equal(new Metadata('---\nfoo: true').at('foo'), true, '.at() with bool')
  t.same(new Metadata('---\nfoo: [1,2,3]').at('foo'), [1, 2, 3], '.at() with array')
  t.same(new Metadata('---\nfoo:\n  bar: 2').at('foo'), { bar: 2 }, '.at() with hash')

  t.equal(new Metadata('---\nfoo: bar').string('foo'), 'bar', '.string() with string')
  t.equal(new Metadata('---\nfoo: 123').string('foo'), null, '.string() with int')
  t.equal(new Metadata('---\nfoo: 123.456').string('foo'), null, '.string() with float')
  t.equal(new Metadata('---\nfoo: true').string('foo'), null, '.string() with bool')
  t.equal(new Metadata('---\nfoo: [1,2,3]').string('foo'), null, '.string() with array')
  t.equal(new Metadata('---\nfoo:\n  bar: 2').string('foo'), null, '.string() with hash')

  t.equal(new Metadata('---\nfoo: bar').bool('foo'), null, '.bool() with string')
  t.equal(new Metadata('---\nfoo: 123').bool('foo'), null, '.bool() with int')
  t.equal(new Metadata('---\nfoo: 123.456').bool('foo'), null, '.bool() with float')
  t.equal(new Metadata('---\nfoo: true').bool('foo'), true, '.bool() with bool')
  t.equal(new Metadata('---\nfoo: [1,2,3]').bool('foo'), null, '.bool() with array')
  t.equal(new Metadata('---\nfoo:\n  bar: 2').bool('foo'), null, '.bool() with hash')

  t.equal(new Metadata('---\nfoo: bar').int('foo'), null, '.int() with string')
  t.equal(new Metadata('---\nfoo: 123').int('foo'), 123, '.int() with int')
  t.equal(new Metadata('---\nfoo: 123.456').int('foo'), null, '.int() with float')
  t.equal(new Metadata('---\nfoo: true').int('foo'), null, '.int() with bool')
  t.equal(new Metadata('---\nfoo: [1,2,3]').int('foo'), null, '.int() with array')
  t.equal(new Metadata('---\nfoo:\n  bar: 2').int('foo'), null, '.int() with hash')

  t.equal(new Metadata('---\nfoo:\n bar:\n  baz: qux').at('foo.bar.baz'), 'qux', 'dotted .at()')
  t.equal(new Metadata('---\nfoo:\n bar:\n  baz: qux').string('foo.bar.baz'), 'qux', 'dotted .string()')
  t.equal(new Metadata('---\nfoo:\n bar:\n  baz: false').bool('foo.bar.baz'), false, 'dotted .bool()')
  t.equal(new Metadata('---\nfoo:\n bar:\n  baz: 123').int('foo.bar.baz'), 123, 'dotted .int()')
})

t.test('specific fields', (t) => {
  t.plan(14)

  t.equal(new Metadata('---\npage: true').page(), true, '.page() with true')
  t.equal(new Metadata('---\npage: false').page(), false, '.page() with false')
  t.equal(new Metadata('').page(), false, '.page() default')
  t.equal(new Metadata('---\ndate: 2010-11-12').page(), false, '.page() with date')

  t.equal(new Metadata('').date(), null, '.date() default')
  t.equal(new Metadata('---\ndate: invalid').date(), null, '.date() invalid')
  t.equal(new Metadata('---\ndate: 2010-01-23').date(), '2010-01-23T00:00:00+00:00', '.date() day')
  t.equal(new Metadata('---\ndate: 2010-01-23T12:34:56Z').date(), '2010-01-23T12:34:56+00:00', '.date() datetime')
  t.equal(new Metadata('---\ndate: 2010-01-23T12:34:56+13:00').date(), '2010-01-22T23:34:56+00:00', '.date() with timezone')

  t.same(new Metadata('').parents(), [], '.parents() with nothing')
  t.same(new Metadata('---\nparent: foo').parents(), ['foo'], '.parents() with single')
  t.same(new Metadata('---\nparents:\n - foo').parents(), ['foo'], '.parents() with single in array')
  t.same(new Metadata('---\nparents:\n - foo\n - bar').parents(), ['foo', 'bar'], '.parents() with multiple in array')
  t.same(new Metadata('---\nparent: foo\nparents:\n - bar').parents(), ['foo'], '.parents() with shadowing')
})
