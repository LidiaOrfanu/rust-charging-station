FROM rust
COPY /target/release/app /usr/local/bin
EXPOSE 8080
CMD [ "app" ]