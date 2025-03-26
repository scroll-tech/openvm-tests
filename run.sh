set -ex -o pipefail

MASK=${1:-01ff}

cargo openvm build
cargo openvm keygen
cargo openvm run --input "$MASK"
cargo openvm prove app --input "$MASK" 2>&1 | tee prove-app.log
cargo openvm prove evm --input "$MASK" 2>&1 | tee prove-evm.log
