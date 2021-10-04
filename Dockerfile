ARG BASE_IMAGE=rust:1.55.0-slim-buster

FROM $BASE_IMAGE as planner
WORKDIR app
RUN cargo install cargo-chef --version 0.1.31
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM $BASE_IMAGE as cacher
WORKDIR app
RUN apt update && apt install -y ca-certificates wget gcc libssl-dev libc6-dev pkg-config
RUN cargo install cargo-chef --version 0.1.31
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM $BASE_IMAGE as builder
WORKDIR app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME
RUN cargo build --release

FROM gcr.io/distroless/cc-debian10
COPY --from=builder /app/target/release/bgsi /
CMD ["./bgsi"]