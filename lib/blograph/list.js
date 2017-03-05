'use strict'
const { List: NativeList } = require('../../native')
const Post = require('./post')

class List {
  constructor (posts) {
    if (posts.toString() === '[object List]') {
      this.impl = posts
    } else {
      this.impl = new NativeList(Array.from(posts).map((post) => {
        if (post.toString() === '[object Post]') {
          return post
        } else if (post instanceof Post) {
          return post.impl
        } else {
          throw new TypeError('Argument is not an array of Post instances')
        }
      }))
    }

    this[Symbol.iterator] = function * () {
      for (const item of this.iterated) {
        yield item
      }
    }.bind(this)
  }

  toArray () {
    return this.impl.toArray().map((post) => new Post(post))
  }

  // Lazy-load the iterator to avoid unnecessary Neon crossings
  get iterated () {
    if (!this._iterated) {
      this._iterated = []
      this.impl.iter((post, prev, next) => {
        this.iterated.push({
          post: new Post(post),
          prev: prev && new Post(prev),
          next: next && new Post(next)
        })
      })
    }

    return this._iterated
  }

  forEach (fun, thisArg = null) {
    return this.iterated.forEach(fun, thisArg)
  }

  map (fun, thisArg = null) {
    return this.iterated.map(fun, thisArg)
  }

  filter (fun, thisArg = null) {
    return new List(
      this.iterated.filter(fun, thisArg).map(({ post }) => post)
    )
  }

  findBySlug (slug) {
    const found = this.iterated.find(({ post }) => post.slug === slug)
    return found ? found.post : null
  }

  sortByDate () {
    return new List(this.iterated.sort(
      ({ post: a }, { post: b }) => (+a.date) - (+b.date)
    ).map(({ post }) => post))
  }

  includes (post, fromIndex = 0) {
    return this.iterated.map(({ post }) => post).includes(post, fromIndex)
  }

  get tags () {
    const tags = new Set()
    for (const { post } of this) {
      post.metadata.tags.forEach((tag) => tags.add(tag))
    }

    return tags
  }

  parentsOf (post) {
    const parents = []
    for (const slug of post.metadata.parents) {
      const parent = this.findBySlug(slug)
      if (parent !== null) {
        parents.push(parent)
      }
    }

    return parents
  }

  childrenOf (post) {
    const children = []
    for (const { post: child } of this) {
      for (const parent of child.metadata.parents) {
        if (parent === post.slug) {
          children.push(child)
        }
      }
    }

    return children
  }
}

module.exports = List
