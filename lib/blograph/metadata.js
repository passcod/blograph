const { Metadata: NativeMetadata } = require('../../native')

class Metadata {
  constructor (yaml) {
    if (typeof yaml === 'string') {
      this.impl = new NativeMetadata(yaml)
    } else {
      this.impl = yaml
    }
  }

  at (dots) {
    return this.impl.at(dots)
  }

  bool (dots) {
    return this.impl.bool(dots)
  }

  int (dots) {
    return this.impl.int(dots)
  }

  string (dots) {
    return this.impl.string(dots)
  }

  get page () {
    return this.impl.page()
  }

  get date () {
    const dt = this.impl.date()
    return dt && new Date(dt)
  }

  get parents () {
    return this.impl.parents()
  }

  get author () {
    return this.string('author')
  }

  get tags () {
    const ts = this.at('tags')
    return Array.isArray(ts) ? ts : []
  }

  get kind () {
    return this.tags[0] || null
  }

  get title () {
    return this.string('title')
  }
}

module.exports = Metadata
