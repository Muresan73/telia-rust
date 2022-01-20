FROM arm64v8/rust

WORKDIR /home/telia

RUN apt-get install libssl-dev pkg-config -y
