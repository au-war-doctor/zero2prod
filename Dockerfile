FROM rust:1.81

# docker will create this app folder if it is not present
WORKDIR /app

# copy all files from working env to the image
COPY . . 

ENV SQLX_OFFLINE=true

RUN cargo build --release

ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./target/release/zero2prod"]