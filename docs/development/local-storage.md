# Local inputs and disposable outputs

`tmp/` and `generated/` are disposable. Required local inputs live elsewhere:

| Location | Contents |
| --- | --- |
| `local/game/` | One preserved original `assets.zip` and the installation configuration `Maybe.toml`. |
| `local/fixtures/` | Fixed accepted image/metadata baselines and legacy MOMI comparison fixtures. |
| `local/tools/fabricator/` | Pinned Fabricator source checkout and `bin/interpreter`. |
| `local/authoring/` | Current authoring sources, plans and metadata; compressed historical helper sources. |
| `extracted/` | Original source corpora, including the stable `test-corpus/` links. |
| `generated/` | Rebuildable packages and review galleries. |
| `tmp/` | Scratch files, temporary executables and disposable installation labs. |

All game material, derived images and local tooling remain ignored by Git.
Back up `local/` and `extracted/` if the accepted historical comparisons must
survive a machine change. A checkout alone does not contain those fixtures.
Keep game mounts outside the two disposable directories.

## Working on another slice

Export from `local/game/assets.zip`, or explicitly select a newer installed
archive. Source hash checks still apply; moving the archive does not update it.
The preserved archive's SHA-256 is
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.

Write new outputs to `generated/` and temporary helpers/labs to `tmp/`.
After artwork is accepted, preserve the character baseline needed by future
tests under a fixed `local/fixtures/<batch>/characters/<character>/` path.
Do not replace old baselines with newly generated output. Preserve reusable
authoring sources outside scratch storage before clearing it.

Slice 186's accepted character outputs and runtime definitions are under
`local/fixtures/slice-186-build/`. Its authoring sources and input records are
under `local/authoring/slice-186/`. These are historical templates, with their
original slice-specific assumptions; adapt them before authoring another batch.
Older Rust, JavaScript and shell helpers are preserved losslessly in
`local/authoring/historical-helpers.tar.gz`.

The accepted slice 186 gallery can be rendered again from its retained fixtures:

```sh
nix-shell --pure --run 'bash local/authoring/slice-186/review.sh'
```

Build a fresh combined package from the preserved game archive with:

```sh
nix-shell --pure --run 'cargo build --locked --release'
target/release/mistria-palette build-characters \
  --archive local/game/assets.zip \
  --characters palettes/sets/characters-trial.json \
  --output generated/characters-trial
```

Use a fresh output path, or remove a previous disposable output first. Historical
development records retain the paths used at the time; their old gallery and
scratch links may no longer exist after cleanup.

## Tests and tools

Follow [testing.md](testing.md) for normal checks and local artwork tests.
`scripts/check-local-assets.sh` uses the retained fixtures and defaults
`FOM_GML_INTERPRETER` to `local/tools/fabricator/bin/interpreter`.
Its browser checks require `FOM_REVIEW_BROWSER` to point to Chromium.
The preserved checkout retains its upstream Git revision. Its binary can be
rebuilt using the pinned development shell if needed:

```sh
nix-shell --pure --run 'cargo build --locked --manifest-path local/tools/fabricator/Cargo.toml --bin interpreter'
cp local/tools/fabricator/target/debug/interpreter local/tools/fabricator/bin/interpreter
```

MOMI smoke tests still require explicitly prepared disposable installation labs;
they do not imply that such a lab survives cleanup. Never use a live installation
as a disposable lab.

## Verification

The 2026-10-13 cleanup reclaimed about 33.4 GiB. All 608,953 retained files
matched their pre-move hashes. The 361 test-file changes were checked against
the committed versions: only path replacements and Rustfmt reflow changed.

Formatting, Clippy, the release build, 398 regular tests, all 359 migrated
artwork tests, three browser tests and three Fabricator tests passed. The latest
gallery was rebuilt from the retained fixtures, checked pixel-for-pixel and
opened in Chromium. Its disposable output occupies about 244 KiB.

The opt-in Adeline archive review test still expects 143 animations, while the
current profile produces 282. The same failure was reproduced with the
unmodified committed test and original path; its assertion was left unchanged.
MOMI installation and live gameplay were not rerun for this storage migration.
Local verification records are under `local/maintenance/storage-migration/`.
