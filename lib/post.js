'use strict'

const cache = require('./cache-getter')
const Metadata = require('./metadata')
const { Post: NativePost } = require('../native')
const sanitize = require('sanitize-html')

class Post {
  constructor (...args) {
    if (args.length === 1) {
      this.impl = args[0]
    } else {
      this.impl = new NativePost(
        args[0],
        args[1] instanceof Metadata ? args[1].impl : args[1],
        args[2]
      )
    }
  }

  get metadata () {
    return cache(this, 'metadata', new Metadata(this.impl.metadata()))
  }

  get isFuture () {
    // Do not cache!
    return this.impl.isFuture()
  }

  get isPage () {
    return cache(this, 'isPage', this.impl.isPage())
  }

  get date () {
    const dt = this.impl.date()
    return cache(this, 'date', dt && new Date(dt))
  }

  get slug () {
    return cache(this, 'slug', this.impl.slug())
  }

  get tags () {
    return cache(this, 'tags', this.metadata.tags)
  }

  get title () {
    return cache(this, 'title', this.impl.title())
  }

  get render () {
    return cache(this, 'render', this.impl.render())
  }

  get excerpt () {
    return cache(this, 'excerpt', sanitize(
      this.render.substr(0, 250).replace(/<$/g, '') // Avoid partial end tags
    , {
      allowedTags: ['b', 'em', 'i', 'strong', 'code'],
      allowedAttributes: [],
      transformTags: {
        h1: 'b',
        h2: 'b',
        h3: 'b',
        h4: 'b',
        h5: 'b',
        h6: 'b',
        // ^ Highlight headers but not in a display:block way

        br: () => ({ tagName: 'i', text: ' /' })
        // ^ In excerpts for poems, add slashes for line breaks
      }
    }).replace(/\s+/g, ' ').replace(/[\w.,…:!?~\-_&%*([/]+$/, '').trim())
    //     No newlines       No partial words nor punctuation.      No trailing space
  }
}

module.exports = Post
