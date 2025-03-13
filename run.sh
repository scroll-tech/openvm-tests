set -e -o pipefail
cargo openvm build
cargo openvm keygen
cargo openvm prove evm 2>&1 |tee prove.log
