require 'active_support/core_ext'
require 'bundler'
require 'date'
require 'pathname'
require 'time'
require 'yaml'
Bundler.require :default

configure do
  set :config, YAML.load_file('config.yml')
  set :cache, Pathname.new('./cache')
  set :views, settings.cache + 'theme'
  set :public_folder, settings.cache + 'theme/static'

  FileUtils.remove_dir settings.cache if settings.cache.exist?
  settings.config['repos'].each do |key, repo|
    repo = "git://github.com/#{repo}.git" if repo =~ %r{^.+/.+$}
    print "Cloning #{repo} "
    cdir = settings.cache + key
    FileUtils.mkdir_p cdir
    repo = Rugged::Repository.clone_at repo, cdir.to_s
    puts '✓'
    settings.config['repos'][key] = repo
  end

  settings.config['domain'] ||= ''
end

helpers do
  def render_post(file, no_render = false)
    candidates = Dir[settings.cache + 'posts' + "#{file}.*"]
    puts file, candidates.inspect
    post = candidates.each do |candidate|
      template = Tilt[candidate]
      file = path2file candidate
      break {
        file: file,
        filedate: DateTime.parse(file.split('-', 4).first(3).join(' ')),
        metadata: {},
        path: candidate,
        raw: IO.read(candidate),
        template: template,
        url: file2url(file)
      } if template
    end
    if post[:raw].start_with? '---'
      matter = post[:raw].split('---', 3).map { |s| s.strip }
      post[:metadata] = YAML.load matter[1]
      post[:raw] = matter[2]
    end
    post[:content] = post[:template].new { post[:raw] }.render unless no_render
    return post
  end

  def a(href, text, options = {})
    options[:rel] = options[:rel] ? "rel=\"#{[options[:rel]].join(' ')}\" " : ""
    options[:title] = options[:title] ? "title=\"#{[options[:title]].join(' ')}\" " : ""
    "<a #{options[:rel]}#{options[:title]}href=\"#{href}\">#{text}</a>"
  end

  def post_info(year, month, day, slug)
    date = [year, month, day].join('-')
    file = [date, slug].join '-'
    {
      file: file,
      date: DateTime.parse(date),
      slug: slug,
      url:  file2url(file)
    }
  end

  def yield_post(year, month, day, slug, options = {})
    post = post_info(year, month, day, slug)
    rendered = render_post post[:file]
    options[:template] ||= :post
    options[:locals] ||= {}
    options[:locals].merge!({
      content: rendered[:content],
      metadata: {
        post: rendered[:metadata],
        site: settings.config['meta']
      },
      date: begin
          DateTime.parse rendered[:metadata]['date']
        rescue
          post[:date]
        end,
      url: {
        slug: post[:slug],
        relative: post[:url],
        absolute: settings.config['domain'] + post[:url]
      },
      title: rendered[:metadata]['title'] ||
        post[:slug].split('-').map { |s|
          s.capitalize
        }.join(' '),
      author: rendered[:metadata]['author'] ||
        settings.config['meta']['author'] || 'anon'
    })
    erb options[:template], options
  end

  def all_posts
    Dir[settings.cache + 'posts' + '*-*-*-*.*'].map { |post|
      path2file post
    }.sort { |x,y|
      x = render_post x, true
      y = render_post y, true
      x = begin DateTime.parse x[:metadata]['date'] rescue x[:filedate] end
      y = begin DateTime.parse y[:metadata]['date'] rescue y[:filedate] end
      x <=> y
    }
  end

  def latest_post
    all_posts.last
  end

  def file2url f
    f = f.split '-', 4
    f.join '/'
  end

  def path2file p
    file = p.split %r{[./]}
    file.slice(file.length - 2)
  end

  def url2file f
    f.gsub '/', '-'
  end
end

before do
  unless request.url.start_with? settings.config['domain']
    settings.config['domain'] = request.url.split('/', 4).first(3).join('/')
  end
end

get '/' do
  erb :index, locals: {
    metadata: {
      site: settings.config['meta']
    } 
  }
end

get '/:year/:month/:day/:slug' do
  yield_post(*%w[year month day slug].map do |k|
    params[k.to_sym]
  end)
end
