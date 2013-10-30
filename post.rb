module Blograph
  class << self
    extend Memoist
    
    def cache; Pathname.new('./cache') end
    def config; YAML.load_file('config.yml') end
    def meta; YAML.load_file(cache + 'posts' + 'metadata.yml') end
    
    memoize :cache, :config, :meta
  end

  class Post
    extend Memoist

    attr_reader :date, :file

    DATE_REGEXP = %r{\d{2,4}[-/]\w{3}[-/]\d{1,2}[-/]?}

    def initialize file
      @file = file.split('posts/').last.sub /\..+?$/, ''
      
      datex = DATE_REGEXP.match @file
      @date = DateTime.parse datex.to_s if datex

      @raw = IO.read path
      @date = DateTime.parse(metadata['date']) if metadata['date']
    end

    def author
      metadata[:author] || Blograph.meta['author'] || 'anon'
    end

    def children
      self.class.all.select { |p| p.parents.include?(self) }
    end
    
    def content
      if @raw.start_with? '---'
        matter = @raw.split('---', 3).map { |s| s.strip }
        matter[2]
      else
        @raw
      end
    end

    def excerpt n = 130
      Sanitize.clean(render, Sanitize::Config::RESTRICTED)
        .gsub(/\s/, ' ').strip[0..n]
    end

    def index
      self.class.all.index self
    end

    def link
      "/#{@date.strftime('%Y/%b/%d').downcase}/#{slug}"
    end

    def metadata
      if @raw.start_with? '---'
        matter = @raw.split('---', 3).map { |s| s.strip }
        YAML.load matter[1]
      else
        {}
      end
    end

    def next
      self.class.all[index + 1] unless index >= self.class.all.length - 1
    end

    def parents
      (metadata['parents'] || []).map do |p|
        self.class.from_link p
      end
    end

    def path
      candidates = Dir[Blograph.cache + 'posts' + "#{@file}.*"]
      post = candidates.each { |c| break c if Tilt[c] }
      post if post.is_a? String
    end

    def previous
      self.class.all[index - 1] unless index <= 0
    end

    def render
      renderer.render
    end

    def renderer
      template.new { content }
    end

    def template
      Tilt[path]
    end

    def tags
      metadata['tags'] || []
    end

    def title
      metadata['title'] || slug.split('-').map { |s|
        s.capitalize
      }.join(' ')
    end

    def slug
      @file.sub DATE_REGEXP, ''
    end

    memoize :author, :children, :content, :excerpt, :index, :link,
      :metadata, :next, :parents, :path, :previous, :render,
      :renderer, :tags, :template, :title, :slug

    class << self
      extend Memoist

      def all
        Dir[Blograph.cache + 'posts' + '*'].reject { |p|
          p =~ /metadata\.yml$/ ||
          p =~ /README\.\w+$/
        }.map { |p|
          p = self.new p
          p if p.path
        }.reject { |p| p.date.nil? }.sort { |x,y|
          x.date <=> y.date
        }
      end

      def from_link str
        all.select { |p| p.link == str || p.link == "/#{str}" }.first
      end

      memoize :all, :from_link
    end
  end
end
