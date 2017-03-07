const compression = require('compression')
const express = require('express')
const helmet = require('helmet')
const logger = require('./logger')
const { load } = require('../lib')
const view = require('./view')

const app = module.exports = express()
app.set('view engine', 'ejs')

app.set('posts', load(process.env.BLOGRAPH_POSTS || './posts').sortByDate())
app.set('frontpage', app.get('posts')
  .filter(({ post }) =>
    (!post.isFuture) &&
    (!post.isPage) &&
    (`${post.metadata.bool('frontpage')}` !== 'false')
  )
  .sortByDate()
)

app.use(logger)
app.use(compression())
app.use(helmet())
app.use(express.static('./public'))
app.use(view)

app.get('/', (req, res) =>
  res.view('index', { posts: app.get('frontpage').reverse() })
)

app.get('/:year/:month/:day/:slug', (req, res, notFound) => {
  const path = req.path.replace(/(^\/|\/$)/g, '')
  const post = req.app.get('posts').findBySlug(path)
  if (!post) { return notFound() }

  let list
  if (req.app.get('frontpage').includes(post)) {
    list = req.app.get('frontpage')
  } else {
    list = req.app.get('posts')
  } // TODO: support more lists (based on query string?)

  let previous = null
  let next = null
  list.forEach(({ post: p, prev, next: n }) => {
    if (post.slug === p.slug) {
      previous = prev
      next = n
    }
  })

  let children = list.childrenOf(post)
  let parents = list.parentsOf(post)

  if (!post.isFuture) {
    // That first one is unlikely now, but there for
    // future-proofing in case of non-chrono lists.
    if (previous && previous.isFuture) { previous = null }
    if (next && next.isFuture) { next = null }

    children = children.filter((p) => !p.isFuture)
    parents = parents.filter((p) => !p.isFuture)
  }

  res.view('post', {
    children,
    list,
    next,
    parents,
    post,
    previous,
    title: post.title
  })
})

app.use((req, res) => res.status(404).view('not-found', { title: 'Not Found' }))
