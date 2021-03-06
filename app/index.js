const cache = require('./cache')
const compression = require('compression')
const express = require('express')
const feed = require('./feed')
const helmet = require('helmet')
const { List } = require('../lib')
const logger = require('./logger')
const { initialLoadError, reclone, reloadPosts } = require('./loader')
const { version } = require('../package.json')
const view = require('./view')

const app = module.exports = express()
app.enable('trust proxy')
app.set('view engine', 'ejs')
app.set('posts', new List([]))

function frontpage () {
  return app.get('posts').filter(({ post }) =>
    (!post.isFuture) &&
    (!post.isPage) &&
    (`${post.metadata.bool('frontpage')}` !== 'false')
  ).sortByDate()
}

setTimeout(() =>
  reclone()
    .then(() => reloadPosts(app))
    .catch((err) => initialLoadError(err))
  , process.env.SLOW_LOAD ? 200 : 0)

app.use(logger)
app.use(compression())
app.use(helmet({ hsts: {
  maxAge: 10886400, // 18 weeks
  preload: true
} }))

app.get('/healthz', (req, res) =>
  req.app.get('posts').length > 0
    ? res.status(204).send()
    : res.status(503).send()
)

app.get('/version', (req, res) => {
  res.type('text/plain')
  res.send(version)
})

app.use(`/assets/${version}/`, express.static('./assets', { maxAge: '1y' }))
app.use(express.static('./content', { maxAge: '24h' }))
app.use(view)

app.post('/hook/reload/posts', (req, res, error) =>
  reclone()
    .then(() => reloadPosts(req.app))
    .then(() => res.status(204).send())
    .catch(error)
)

app.get('/', cache('10m'), (req, res) =>
  res.view('index', { posts: frontpage().reverse })
)

app.get('/feed', cache('1h'), (req, res) => {
  res.type('application/rss+xml')
  res.send(feed(frontpage().reverse, {
    title: 'Félix “passcod” Saparelli — Blog',
    description: 'Feed of the front page of @passcod’s blog',
    feed_url: 'https://blog.passcod.name/feed'
  }))
})

app.get('/tag/:tag', cache('1h'), (req, res) => {
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
      .reverse
  })
})

// Any other GET is potentially a post or page
app.get('/*', cache('1h'), (req, res, notFound) => {
  // FIXME: Support dated subpaths
  const path = req.path.replace(/(^\/|\/$)/g, '')
  let post = req.app.get('posts').findBySlug(path)
  if (!post) { post = req.app.get('posts').findBySlug('/' + path) }
  if (!post) { return notFound() }

  let list
  if (frontpage().includes(post)) {
    list = frontpage()
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

  /* istanbul ignore if */
  if (!post.isFuture) {
    // That first one is unlikely now, but there for
    // future-proofing in case of non-chrono lists.
    /* istanbul ignore next */
    if (previous && previous.isFuture) { previous = null }
    /* istanbul ignore next */
    if (next && next.isFuture) { next = null }
    // TODO: disable the istanbul ignores when non-frontpage
    // lists are implemented.

    children = children.filter(({ post: p }) => !p.isFuture)
    parents = parents.filter(({ post: p }) => !p.isFuture)
  } else {
    res.cacheControl = { noCache: true }
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

app.use((req, res) => res
  .status(404)
  .view('not-found', {
    title: 'Not Found'
  })
)
