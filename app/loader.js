const chalk = require('chalk')
const git = require('simple-git')()
const { load } = require('../lib')
const rimraf = require('rimraf')

function rmrf (path) {
  return new Promise((resolve, reject) =>
    rimraf(path, (err) => {
      if (err) {
        reject(err)
      } else {
        resolve()
      }
    })
  )
}

function reclone () {
  if (process.env.BLOGRAPH_POSTS) {
    console.error(chalk.red('ERR') + ' BLOGRAPH_POSTS is set')
    console.error(chalk.red('ERR') + ' not clobbering, not recloning')
    return Promise.resolve()
  }

  if (!process.env.BLOGRAPH_REPO) {
    console.error(chalk.bold.red('FATAL: Unrecoverable'))
    console.error(chalk.bold.red('FATAL: ') + chalk.bold('neither BLOGRAPH_REPO nor BLOGRAPH_POSTS available'))
    console.error(chalk.bold.red('FATAL: ') + chalk.bold('cannot load posts, aborting'))
    process.exit(1)
  }

  console.log(chalk.blue('INFO') + ' Deleting ./posts')
  return rmrf('./posts')
  .then(() => console.log(chalk.blue('INFO') + ' Cloning ' + process.env.BLOGRAPH_REPO))
  .then(() => git.clone(process.env.BLOGRAPH_REPO, './posts'))
  .then(() => console.log(chalk.blue('INFO') + chalk.green(' Done cloning')))
}

function reloadLists (app) {
  console.log(chalk.blue('INFO') + ' Loading posts using Rust lib')
  const posts = load(process.env.BLOGRAPH_POSTS || './posts').sortByDate()
  console.log(chalk.blue('INFO') + chalk.green(' Done loading posts'))

  app.set('posts', posts)
  app.set('frontpage', posts
    .filter(({ post }) =>
      (!post.isFuture) &&
      (!post.isPage) &&
      (`${post.metadata.bool('frontpage')}` !== 'false')
    )
    .sortByDate()
  )
}

module.exports.reclone = reclone
module.exports.reloadLists = reloadLists
