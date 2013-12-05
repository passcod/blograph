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

  if %w[posts theme].reduce(false) do |memo, k|
    envvar = ENV["BLOGRAPH_#{k.upcase}"]
    Blograph.config['repos'][k] = envvar unless envvar.nil? or envvar.empty?
    memo || Blograph.config['repos'][k].empty?
  end
    raise 'Repositories aren´t set up, can´t start up.'
  end

  Blograph.config['repos'].keys.each { |r| Blograph.fetch_repo(r) }
end

helpers do
  def a(href, text, o = {})
    o[:rel] = o[:rel] ? "rel=\"#{[o[:rel]].join(' ')}\" " : ""
    o[:title] = o[:title] ? "title=\"#{[o[:title]].join(' ')}\" " : ""
    "<a #{o[:rel]}#{o[:title]}href=\"#{href}\">#{text}</a>"
  end

  def yield_post(post, options = {})
    options[:ref] ||= 'master'
    post = Blograph::Post.from_link(post, options[:ref]) if post.is_a? String

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

get '/@:ref/' do
  redirect "/@#{params[:ref]}"
end

get '/@:ref' do
  erb :index, locals: {
    posts: Blograph::Post.all(params[:ref])
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
    end,
    title: "Posts tagged as #{params[:tag]}"
  }
end

get '/:year/:month/:day/:slug' do
  yield_post %w[year month day slug].map { |k| params[k.to_sym] }.join('/')
end

get '/@:ref/:year/:month/:day/:slug' do
  yield_post %w[year month day slug].map { |k| params[k.to_sym] }.join('/'),
    ref: params[:ref]
end

post '/hook/reload/:repo' do
  Blograph.fetch_repo params[:repo]
  if params[:repo] == 'posts'
    Blograph.reset_memoiz
  elsif params[:repo] == 'theme'
    puts 'Clear template cache'
    template_cache.clear
  end
end
