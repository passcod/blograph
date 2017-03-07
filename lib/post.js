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
    return new Metadata(this.impl.metadata())
  }

  get isFuture () {
    return this.impl.isFuture()
  }

  get isPage () {
    return this.impl.isPage()
  }

  get date () {
    const dt = this.impl.date()
    return dt && new Date(dt)
  }

  get slug () {
    return this.impl.slug()
  }

  get title () {
    return this.impl.title()
  }

  get render () {
    return this.impl.render()
  }

  get excerpt () {
    //                                          Avoid partial end tags
    return sanitize(this.render.substr(0, 200).replace(/<$/g, ''), {
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
    }).replace(/\s+/g, ' ').replace(/[\w.,…:!?~\-_&%*([/]+$/, '').trim()
    //     No newlines       No partial words nor punctuation.      No trailing space
  }
}

module.exports = Post
