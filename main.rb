require 'active_support/core_ext'
require 'bundler'
require 'pathname'
require 'time'
require 'yaml'
Bundler.require :default

require './post'

configure do
  set :views, Blograph.cache + 'theme'
  set :public_folder, Blograph.cache + 'theme/static'
  
  Blograph.config['repos'].keys.each { |r| Blograph.fetch_repo(r) }
end

helpers do
  def a(href, text, o = {})
    o[:rel] = o[:rel] ? "rel=\"#{[o[:rel]].join(' ')}\" " : ""
    o[:title] = o[:title] ? "title=\"#{[o[:title]].join(' ')}\" " : ""
    "<a #{o[:rel]}#{o[:title]}href=\"#{href}\">#{text}</a>"
  end

  def yield_post(post, options = {})
    post = Blograph::Post.from_link(post) if post.is_a? String

    options[:template] ||= :post
    options[:locals] ||= {}
    options[:locals].merge!({
      post: post,
      title: post.title
    })
    erb options[:template], options
  end
end

get '/' do
  erb :index, locals: {
    posts: Blograph::Post.all,
  }
end

get '/tag/:tag' do
  erb :tag, locals: {
    posts: Blograph::Post.all.select do |post|
      post.tags.include? params[:tag]
    end,
    tag: params[:tag],
    taginfo: if Blograph.meta['tags']
      Blograph.meta['tags'][params[:tag]]
    end
  }
end

get '/:year/:month/:day/:slug' do
  yield_post %w[year month day slug].map { |k| params[k.to_sym] }.join('/')
end

post '/github-hook/:repo' do
  Blograph.fetch_repo params[:repo]
  if params[:repo] == 'posts'
    Blograph.reset_memoiz
  elsif params[:repo] == 'theme'
    puts 'Clear template cache'
    template_cache.clear
  end
end
