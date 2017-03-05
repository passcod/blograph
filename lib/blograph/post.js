const Metadata = require('./metadata')
const { Post: NativePost } = require('../../native')

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
}

module.exports = Post
