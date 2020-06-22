FROM rust:1.44.0
EXPOSE 8080
WORKDIR /usr/src/midasSocialAPI
COPY . .
RUN cargo install --path .
CMD ["midas_social"]