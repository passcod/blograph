const chalk = require('chalk')
const git = require('simple-git')()
const { load } = require('../lib')
const rmrf = require('promisify-es6')(require('rimraf'))

async function reclone () {
  if (process.env.BLOGRAPH_POSTS) {
    console.error(chalk.red('ERR') + ' BLOGRAPH_POSTS is set')
    console.error(chalk.red('ERR') + ' not clobbering, not recloning')
    return
  }

  if (!process.env.BLOGRAPH_REPO) {
    throw new Error('Neither BLOGRAPH_REPO nor BLOGRAPH_POSTS available')
  }

  console.log(chalk.blue('INFO') + ' Deleting ./posts')
  await rmrf('./posts')

  console.log(chalk.blue('INFO') + ' Cloning ' + process.env.BLOGRAPH_REPO)
  await git.clone(process.env.BLOGRAPH_REPO, './posts', ['--recursive', '-j4'])

  console.log(chalk.blue('INFO') + chalk.green(' Done cloning'))
}

function reloadPosts (app) {
  console.log(chalk.blue('INFO') + ' Loading posts using Rust lib')
  const posts = load(process.env.BLOGRAPH_POSTS || './posts').sortByDate()
  console.log(chalk.blue('INFO') + chalk.green(' Done loading posts'))
  app.set('posts', posts)
}

function initialLoadError (err) {
  console.error(chalk.bold.red('FATAL: Unrecoverable'))

  const errorLines = err.stack.toString().split(/\r?\n/)
  console.error(chalk.bold.red('FATAL: ') + chalk.bold(errorLines.shift()))

  for (const line of errorLines) {
    console.error(chalk.bold.red('FATAL: ') + line)
  }

  process.exit(1)
}

module.exports.reclone = reclone
module.exports.reloadPosts = reloadPosts
module.exports.initialLoadError = initialLoadError
