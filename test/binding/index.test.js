const binding = require('../../native')
const t = require('tap')

t.test((t) => {
  t.plan(4)
  t.type(binding.load, 'function', 'load()')
  t.type(binding.List, 'function', 'new List()')
  t.type(binding.Metadata, 'function', 'new Metadata()')
  t.type(binding.Post, 'function', 'new Post()')
})
