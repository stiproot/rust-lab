cargo new restaurant --lib --vcs none

cargo test -- --test-threads=1
cargo test -- --show-output
cargo test one_hundred
cargo test --test integration_test
cargo test -- --ignored
cargo test -- --include-ignored

# We can specify part of a test name, and any test whose name matches that value will be run. 
# For example, because two of our tests’ names contain add, we can run those two by running cargo test add:
cargo test add


IGNORE_CASE=1 cargo run -- to poem.txt
cargo run -- to poem.txt > output.txt

cargo build --release
cargo doc
cargo doc --open

ccargo login
# > abcdefghijklmnopqrstuvwxyz012345
# This command will inform Cargo of your API token and store it locally in ~/.cargo/credentials.

cargo publish
cargo yank --vers 1.0.1
cargo yank --vers 1.0.1 --undo

cargo install ripgrep
