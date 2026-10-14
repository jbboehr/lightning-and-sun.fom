#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."
: "${FOM_REVIEW_BROWSER:?Set FOM_REVIEW_BROWSER to a Chromium executable}"

# Artwork tests use accepted fixtures. MOMI tests need an explicitly prepared
# installation lab and are run separately.
targets=()
for source in tests/*.rs; do
  [[ "$source" == tests/momi_install.rs ]] && continue
  if grep -q '"local/fixtures/' "$source"; then
    target="${source#tests/}"
    targets+=(--test "${target%.rs}")
  fi
done
test "${#targets[@]}" -gt 0
cargo test --release --locked "${targets[@]}" -- --ignored
FOM_GML_INTERPRETER="${FOM_GML_INTERPRETER:-$PWD/local/tools/fabricator/bin/interpreter}" \
  cargo test --release --locked --test gml_runtime -- --ignored
cargo test --release --locked --test review_batch -- --ignored
