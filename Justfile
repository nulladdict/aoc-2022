default:
  just --list

alias r := run
# run in release profile
run day *args:
  cargo run -p {{day}} --release {{args}}

alias d := debug
# run in debug profile
debug day *args:
  cargo run -p {{day}} {{args}}

alias rn := run-nightly
# run with nightly toolchain
run-nightly day *args:
  cargo +nightly run -p {{day}} --release {{args}}

alias dn := debug-nightly
# debug with nightly toolchain
debug-nightly day *args:
  cargo +nightly run -p {{day}} {{args}}
