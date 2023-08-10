FROM rust AS build
COPY . ./build
WORKDIR /build
RUN cargo build --release

FROM rust AS runtime
COPY --from=build /build/target/release/freight /usr/bin/freight
CMD ["freight"]

