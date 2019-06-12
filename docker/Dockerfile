FROM rust:1.35-slim

RUN rustup update nightly \
 && rustup default nightly

ENV USER=root
WORKDIR /opt/app

CMD [ "tail", "-f", "/dev/null" ]
