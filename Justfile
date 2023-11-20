# This causes the .env file to be read by Just.
set dotenv-load := true

# Allow for positional arguments in Just receipes.
set positional-arguments := true

# Default recipe that runs if you type "just".
default: fmt clippy tests doctests

# Formats and checks rust files
fmt: fmt-write fmt-check

fmt-write: # nightly format all rust files 
  cargo +nightly fmt --all

fmt-check: # nightly check formatting of all rust files
  cargo +nightly fmt --all --check

clippy: # nightly run clippy on all rust files
  cargo +nightly clippy --all-targets --all-features -- -D warnings

tests: # run all tests
  cargo +nightly nextest run \
    --locked \
    --all-features \
    --workspace \
    --exclude examples

doctests: # run all doctests
  cargo +nightly test \
    --locked \
    --doc \
    --all-features \
    --workspace \
    --exclude examples

axt: # run axt
  cd bins/axt && cargo +nightly run -- -vvv
