FROM rust:1.44.1 AS builder

WORKDIR /usr/src

# download target for static linking
RUN rustup target add x86_64-unknown-linux-musl

# create a temp project and build the app's dependencies.
# If the Cargo.toml and Cargo.lock files have not changed,
# we can use the docker build cache and skip these.
RUN USER=root cargo new automat
WORKDIR /usr/src/automat
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source and build the program.
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# statically linked binary in a scratch container
FROM scratch
COPY --from=builder /usr/local/cargo/bin/atm .
CMD ["./atm"]
