[![Code Climate](https://codeclimate.com/github/passcod/blograph.png)](https://codeclimate.com/github/passcod/blograph)
[![Built with Heroku in mind](https://d1lpkba4w1baqt.cloudfront.net/heroku-logo-light-88x31.png)](https://heroku.com)
[![Powered by Puma](http://i.imgur.com/nFmB4vq.png)](http://puma.io)

## Notes:

- Blograph is blogging done my way. Let's not use superlatives just yet.

- Blograph does NOT handle minification. That task should be done
  either by a Rack Middleware, by a service higher on the HTTP reverse
  proxy chain on your server, or by an external service like Cloudflare.

- Blograph was created for the singular purpose of running my blog.
  Fortunately, I often painstakingly work to make my software generic,
  so you can use it without changing a single line of code.

- Blograph is the source of truth for the information it displays.
  Or should be. What you do in your own home is the business of other
  residents, pets, some parts of the law, and your mom. Don't mind me.

- Blograph handles more markup languages than I currently use.
  Except for the theme where it uses ERB all the time and yes I'll
  find a way to make this generic yes I will. \*glowers at Sinatra\*

- Blograph is not designed for static website generation, and thus
  cannot and should not be used to publish to Github Pages, Dropbox,
  or S3. Get a server or put it on Heroku or something.

- Blograph is in fact designed with Heroku in mind. Clone it, run
  `heroku create`, then set the config vars BLOGRAPH_POSTS and
  BLOGRAPH_THEME to point to repos containing your posts and theme.
  See https://github.com/passcod/blograph-theme and
  https://github.com/passcod/blograph-posts for a real-life example.

- Blograph can also read from a config.yml in the root for these
  settings. Env/config vars overrule config.yml.

- Blograph can read any git://... repo. It doesn't have to be on
  github. It HAS to be git:// though, at least until libgit2 can
  support other things easily. Repos have to be public (again, this
  is until libgit2 supports more stuff).

- Blograph can be hit with POST requests (content doesn't matter) to
  /hook/reload/posts and /hook/reload/theme, and will trigger a reload
  of the repos, so you can set up your blog to be updated when you push
  to git without ever restarting the server.

- Blograph would prefer to use Rubinius all the time but Heroku doesn't
  like Rubinius 2 at the moment. Edit 4 Apr 2014: Rubinius support
  removed until I investigate this again.

- Blograph uses Puma.

- Blograph can and will show older versions of articles if that is so
  requested. If you do not want your typos or factual innacuracies to
  show up, use --amend and rebase.

- Blograph does not use SASS, LESS, Stylus, or other CSS preprocessors.
  If you think you can convince me otherwise, shoot me an email at
  blograph@passcod.name. If you do convince me, I will reply.

- Blograph uses YAML Front Matter.

- Blograph is dedicated to the Public Domain, see Creative Commons 0:
  https://creativecommons.org/publicdomain/zero/1.0/ or the LICENSE file.
