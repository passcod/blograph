const compression = require('compression')
const enforceHttps = require('express-enforces-ssl')
const express = require('express')
const helmet = require('helmet')
const { List } = require('../lib')
const logger = require('./logger')
const { reclone, reloadLists } = require('./loader')
const view = require('./view')

const app = module.exports = express()
app.set('view engine', 'ejs')

app.set('posts', new List([]))
app.set('frontpage', new List([]))

reclone()
.then(() => reloadLists(app))
.catch((err) => { throw err })

app.use(logger)
app.use(compression())

app.enable('trust proxy')
app.use(enforceHttps())
app.use(helmet({ hsts: {
  maxAge: 10886400, // 18 weeks
  preload: true
} }))

app.use(express.static('./public'))
app.use(view)

app.post('/hook/reload/posts', (req, res) =>
  reclone()
  .then(() => reloadLists(req.app))
  .then(() => res.json({ ok: true }))
  .catch((err) => { throw err })
)

app.get('/', (req, res) =>
  res.view('index', { posts: app.get('frontpage').reverse() })
)

app.get('/tag/:tag', (req, res) => {
  const { tag } = req.params
  res.view('tag', {
    tag,
    title: `Tag: ${tag}`,
    posts: app.get('posts')
      .filter(({ post }) =>
        (post.tags.includes(tag)) &&
        (!post.isFuture)
      )
      .sortByDate()
      .reverse()
  })
})

// Any other GET is potentially a post or page
app.get((req, res, notFound) => {
  const post = req.app.get('posts').findBySlug(req.path)
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

  let children = req.app.get('posts').childrenOf(post)
  let parents = req.app.get('posts').parentsOf(post)

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

// 404 on GET returns a view
app.get((req, res) => res
  .status(404)
  .view('not-found', {
    title: 'Not Found'
  })
)

// 404 on any other method returns nothing
app.use((req, res) => res.status(404))
