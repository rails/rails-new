FROM buildpack-deps:bookworm

ARG RUBY_VERSION=3.3.4
ARG NODE_VERSION=22.1
ARG YARN_VERSION=1.22.22
ARG RAILS_VERSION

RUN curl https://mise.run | sh

ENV MISE_HOME /root/.local
ENV PATH $MISE_HOME/bin:$MISE_HOME/share/mise/shims:/usr/local/bin:$PATH

RUN mise use --global "ruby@$RUBY_VERSION" && \
  mise use --global "node@$NODE_VERSION" && \
  mise use --global "yarn@$YARN_VERSION"

# Install Rails based on the version specified but if not specified, install the latest version.
RUN if [ -z "$RAILS_VERSION" ] ; then gem install rails ; else gem install rails -v $RAILS_VERSION ; fi

