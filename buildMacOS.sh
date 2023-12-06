rustup target add x86_64-apple-darwin
cargo build --target=x86_64-apple-darwin --release
cd ./target/x86_64-apple-darwin/release && tar -c -a -f ../../../x86_64-apple-darwin.tar.gz ./boilit