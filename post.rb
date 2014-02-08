module Blograph
  class << self
    extend Memoist
    
    def cache
      Pathname.new('./cache')
    end
    
    def config
      begin
        YAML.load_file('config.yml')
      rescue
        {'repos' => {}}
      end
    end
    
    def meta
      begin
        YAML.load_file(cache + 'posts' + 'metadata.yml')
      rescue
        { 'title' => 'Blograph' }
      end
    end

    def reset_memoiz
      puts "Resetting memoization"
      Blograph::Post.flush_cache
      meta true # Force metadata reload
    end

    def fetch_repo repo
      return unless Blograph.config['repos'].keys.include? repo
      target = Blograph.cache + repo 

      remote = Blograph.config['repos'][repo]
      remote = remote.remotes.first.url if remote.is_a? Rugged::Repository
      remote = "git://github.com/#{remote}.git" if remote =~ %r{^[-_\.a-zA-Z0-9]+/[-_\.a-zA-Z0-9]+$}

      FileUtils.remove_dir target if target.exist?
      FileUtils.mkdir_p target

      print "Cloning #{remote} into #{target} "
      local = Rugged::Repository.clone_at remote, target.to_s
      puts '✓'

      Blograph.config['repos'][repo] = local
    end
    
    memoize :cache, :config, :meta
  end

  class Post
    extend Memoist

    attr_reader :date, :file
    attr_accessor :template_options

    DATE_REGEXP = %r{\d{2,4}[-/]\w{3}[-/]\d{1,2}[-/]?}

    def initialize file, ref = 'master'
      @ref = ref
      @file = file.split('posts/').last.sub /\..+?$/, ''
      
      datex = DATE_REGEXP.match @file
      @date = DateTime.parse datex.to_s if datex

      @raw = IO.read path
      @date = DateTime.parse(metadata['date']) if metadata['date']
      @template_options = {
        autolink: true,
        disable_indented_code_blocks: true,
        fenced_code_blocks: true,
        footnotes: true,
        highlight: true,
        smartypants: true,
        space_after_header: true,
        strikethrough: true,
        superscript: true,
        tables: true
      }
    end

    def all
      self.class.all @ref
    end

    def author
      metadata[:author] || Blograph.meta['author'] || 'anon'
    end

    def children
      all.select { |p| p.parents.include?(self) }
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

    def future?
      DateTime.now < @date
    end

    def index
      all.index { |p| p.link == link }
    end

    def link
      base = "/#{@date.strftime('%Y/%b/%d').downcase}/#{slug}"
      if @ref != 'master'
        "/@#{@ref}" + base
      else
        base
      end
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
      all[index + 1] unless index >= all.length - 1
    end

    def parents
      (metadata['parents'] || []).map do |p|
        self.class.from_link p, @ref
      end
    end

    def path file = @file
      self.class.switch_ref @ref
      candidates = Dir[Blograph.cache + 'posts' + "#{file}.*"]
      post = candidates.each { |c| break c if Tilt[c] }
      if post.is_a? String
        post
      elsif file == "404"
        Pathname.new("./default-404.md").to_s
      else
        path "404"
      end
    end

    def previous
      all[index - 1] unless index <= 0
    end

    def render
      renderer.render
    end

    def renderer
      template.new template_options do content end
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

    memoize :author, :children, :content, :excerpt, :future?,
      :index, :link, :metadata, :next, :parents, :path, :previous,
      :render, :renderer, :tags, :template, :title, :slug

    class << self
      extend Memoist

      def all ref = 'master'
        self.switch_ref ref
        Dir[Blograph.cache + 'posts' + '*'].reject { |p|
          p =~ /metadata\.yml$/ ||
          p =~ /README\.\w+$/
        }.map { |p|
          p = self.new p, ref
          p if p.path
        }.reject { |p| p.date.nil? }.sort { |x,y|
          x.title <=> y.title
        }.sort { |x,y|
          d = x.date <=> y.date
          if d == 0
            x.title <=> y.title
          else
            d
          end
        }
      end

      def from_link str, ref = 'master'
        all(ref).select do |p|
          if ref == 'master'
            p.link == str || p.link == "/#{str}"
          else
            p.link == "/@#{ref}#{str}" || p.link == "/@#{ref}/#{str}"
          end
        end.first
      end

      def switch_ref ref = 'master'
        # Goes through system for now, until I figure
        # out how to do it using pure Rugged/libgit2.
        changed = system "cd #{Blograph.cache + 'posts'}; git checkout #{ref}"
        switch_ref unless changed
        return changed
      end

      memoize :all, :from_link
    end
  end
end
