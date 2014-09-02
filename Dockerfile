FROM ruby
MAINTAINER Félix Saparelli

EXPOSE 80
CMD ["rackup -s puma -p 80"]
