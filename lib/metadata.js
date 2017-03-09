'use strict'
const cache = require('./cache-getter')
const { Metadata: NativeMetadata } = require('../native')

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
    return cache(this, 'page', this.impl.page())
  }

  get date () {
    const dt = this.impl.date()
    return cache(this, 'date', dt && new Date(dt))
  }

  get parents () {
    return cache(this, 'parents', this.impl.parents())
  }

  get author () {
    return cache(this, 'author', this.string('author'))
  }

  get tags () {
    const ts = this.at('tags')
    return cache(this, 'tags', Array.isArray(ts) ? ts : [])
  }

  get kind () {
    return cache(this, 'kind', this.tags[0] || null)
  }

  get title () {
    return cache(this, 'title', this.string('title'))
  }
}

module.exports = Metadata
