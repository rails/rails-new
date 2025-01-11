ARG RUBY_VERSION=3.4.1
FROM ruby:${RUBY_VERSION}
ARG RAILS_VERSION
# Install Rails based on the version specified but if not specified, install the latest version.
RUN if [ -z "$RAILS_VERSION" ] ; then gem install rails ; else gem install rails -v $RAILS_VERSION ; fi
