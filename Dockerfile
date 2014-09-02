FROM ruby
MAINTAINER Félix Saparelli

EXPOSE 80
CMD ["bundle", "exec", "rackup", "-s", "puma", "-p", "80"]
