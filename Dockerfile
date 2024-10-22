FROM rust:1.81-slim 

# copy binary over
COPY /target/release/zero2prod zero2prod 

ENV SQLX_OFFLINE=true

COPY configuration configuration

ENV APP_ENVIRONMENT=production

ENTRYPOINT ["./zero2prod"]