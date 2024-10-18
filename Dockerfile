FROM rust:1.81 AS builder

# docker will create this app folder if it is not present
WORKDIR /app

# copy all files from working env to the image
COPY . . 

ENV SQLX_OFFLINE=true
RUN cargo build --release

# runtime stage
FROM rust:1.81-slim AS runtime

# I guess we need to re-declare the workdir for some reason??
WORKDIR /app

# for some reason it's better to COPY the compiled 
# binary from the builder to the runtime??
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration

ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./zero2prod"]