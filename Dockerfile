ARG RUBY_VERSION=3.4.1
FROM ruby:${RUBY_VERSION}

ARG NODE_VERSION=v22.6.0
ARG YARN_VERSION=1.22.22

RUN curl -fsSL https://nodejs.org/dist/$NODE_VERSION/node-$NODE_VERSION-linux-x64.tar.gz \
      -o /tmp/node-$NODE_VERSION-linux-x64.tar.gz \
  && tar -xzvf /tmp/node-$NODE_VERSION-linux-x64.tar.gz -C /usr/local \
  && rm -rf /tmp/node-$NODE_VERSION-linux-x64.tar.gz

ENV PATH=/usr/local/node-$NODE_VERSION-linux-x64/bin:$PATH

RUN npm install -g yarn@$YARN_VERSION

ARG RAILS_VERSION
# Install Rails based on the version specified but if not specified, install the latest version.
RUN if [ -z "$RAILS_VERSION" ] ; then gem install rails ; else gem install rails -v $RAILS_VERSION ; fi
