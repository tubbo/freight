FROM rust:alpine AS build
COPY . ./build
WORKDIR /build
RUN cargo build --release

FROM alpine
COPY --from=build /build/target/release/freight /usr/bin/freight
CMD ["freight"]
