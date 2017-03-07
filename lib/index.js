const List = require('./list')
const { load } = require('../native')
const Metadata = require('./metadata')
const Post = require('./post')

module.exports = { List, load: (base) => new List(load(base)), Metadata, Post }
