const pkg = require('../package.json')
const RSS = require('rss')

const METADATA = {
  generator: `${pkg.name}/${pkg.version}`,
  site_url: 'https://blog.passcod.name',
  image_url: 'https://secure.gravatar.com/avatar/a794d473626fc9d329a035c38e2fb832',
  copyright: `© 2008-${new Date().getUTCFullYear()}. License: CC-BY 4.0 International.`,
  language: 'en_NZ',
  ttl: 360 // 6 hours
}

function item (post) {
  return {
    title: post.title,
    description: post.excerpt,
    url: 'https://blog.passcod.name/' + post.slug,
    date: post.date
  }
}

module.exports = function feed(posts, meta) {
  const feed = new RSS(Object.assign({}, METADATA, meta))

  posts.forEach(({ post }) => feed.item(item(post)))

  return feed.xml({ indent: true })
}
