ARG RUBY_VERSION=3.2.3
FROM ruby:${RUBY_VERSION}
ARG USER_ID=1000
ARG GROUP_ID=1000
RUN groupadd -g $GROUP_ID app -o && useradd -u $USER_ID -g app -m app -o
USER app
ARG RAILS_VERSION
# Install Rails based on the version specified but if not specified, install the latest version.
RUN if [ -z "$RAILS_VERSION" ] ; then gem install rails ; else gem install rails -v $RAILS_VERSION ; fi
