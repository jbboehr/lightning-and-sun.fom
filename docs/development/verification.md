# Verification record

2026-09-06. Rust port reliability verdict: **PASS_WITH_RESIDUAL_RISK** for the
offline proof of concept. The later [portrait toggle verification](portrait-toggle.md)
adds in-game evidence. Finished art remains outside the experiment.

## Repeatable automated checks

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
```

The final formatting, Clippy, test, and release-build checks passed through both
Nix entry points. All 17 synthetic pipeline tests passed.
The normal suite uses only synthetic PNGs, metadata, palettes, and miniature ZIPs.
The separate `momi_install` test is opt-in because it requires local game files
and an installed isolated lab. Run it using [the lab procedure](momi-lab.md),
before uninstalling.

For a checkout whose Nix files are still untracked, evaluate the flake using only
copies of its three configuration files. Do not use `path:.` over the repository,
because that would include ignored game files in the Nix source snapshot.

```sh
mkdir -p tmp/nix-shell
cp flake.nix flake.lock shell.nix tmp/nix-shell/
nix develop path:./tmp/nix-shell --command bash -c \
  'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix flake check --no-build --all-systems path:./tmp/nix-shell
```

Both shell entry points use the same Fenix toolchain and Cargo lock. Fenix is
pinned at `a557691643f3b1ab520474d20ba2491d957ef729`, with its Nixpkgs input following
the project's lock. Rust and Cargo are 1.98.1. The pure shell contains no Python
interpreter. Both x86_64-linux and aarch64-linux shell definitions evaluate;
execution is checked on x86_64 Linux only.

## Rust port behavior

All five commands are native Rust. The old source scripts and unittest suite are
removed. Tests cover simultaneous replacements, partial alpha, transparent RGB,
vanilla byte preservation, deterministic reports and images, copied metadata,
output separation, symlinks, failed-input preflight, mismatched inventories,
nearest-neighbor scaling, selected ZIP exports, package contents, two-asset limits,
and frame-strip dimensions. Indexed transparency, RGB, grayscale, and grayscale
alpha have decoded-pixel checks; APNG is rejected.

A duplicate-member regression initially failed: the first ZIP reader collapsed
same-name central-directory entries before the exporter could count them.
The exporter now uses `rc-zip-sync`, which exposes the entries individually;
the regression passes. The `zip` crate is used only to write synthetic fixtures.

The package preserves an earlier installer regression test: its deployable tree
contains only `manifest.toml` and changed `images/replace/spr_*.png` files. MOMI
interprets root JSON as game content, so the package report goes only to stdout.
The default manifest is embedded in the binary; explicit manifests remain supported.

Contact-sheet labels now use an embedded 8×8 bitmap font. Changed PNG encoding
also differs from the former tool, so compressed-byte hashes can change even
when decoded pixels match. Vanilla PNG bytes remain exact. Neither output depends
on a system font or image-processing runtime.

## Review findings

Independent review and a separate test pass exposed a file/directory collision:
an input folder named `palette-report.json` conflicts with the generated report
and previously left partial output. A second fixture covers conflicting export
paths. Both regressions failed before the fix and passed afterward. The shared
writer now checks the complete output namespace before creating directories.

The review also noted that replacing an input archive concurrently can make its
reported hash describe a different file from the one opened for extraction. This
behavior predates the port and was left outside this change. The recorded workflow
uses an unchanged local copy; concurrent input updates remain unsupported.

## Real asset comparison

The untouched archive's SHA-256 is:

```text
b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5
```

The Rust exporter produced the same original PNG and metadata hashes. Applying
the stylized palette changed 4,000 pixels: 2,439 `#E3A17BFF`, 1,231 `#C47054FF`,
and 330 `#9F5544FF`. The two-frame canvas stayed 592×180 and alpha was unchanged.
Comparing the earlier generated study against the Rust result reported zero
RGBA differences and identical sidecar bytes. The Rust PNG's SHA-256 is
`4b898a85f9a999599b35d05077d002ff8f9802fa0112e450d78a457612601dc5`.
Repeat palette runs have identical reports and PNG hashes. Vanilla keeps the
original PNG bytes. The 4x Rust contact sheet was visually inspected.

Local evidence is ignored by Git:

```text
extracted/adeline-rust/export-report.json
generated/adeline-rust-stylized/palette-report.json
generated/contact-sheet-adeline-rust-stylized.png
generated/contact-sheet-adeline-rust-stylized.json
generated/momi-build-report.json
tmp/momi-install.log
tmp/momi-verification.json
tmp/momi-uninstall.log
```

## Isolated MOMI round trip

MOMI v0.15.10 installed the Rust-built package into a bubblewrap-isolated archive
copy. The log identified `/lab`, named the portrait replacement, and reported one
installed mod. The opt-in Rust check passed:

```sh
nix-shell --pure --run 'cargo test --locked --test momi_install installed_adeline_frames_and_metadata_match -- --ignored --nocapture'
```

Both 296×180 frames reconstructed exactly from the installed atlas, including their
transparent canvas. The selected animation's parsed metadata and other spring
atlas placements were unchanged. Other rewritten metadata files were semantically
equal. The only added archive member was MOMI's `manifest.toml` marker; no member
was removed. Only the expected atlas PNG changed among non-metadata members.

The sandbox then ran MOMI with `--uninstall`. Both `cmp` comparisons passed, and
SHA-256 matched the original for the source archive, restored lab archive, and
`assets.bak.zip`. The lab is left restored to vanilla.

## Remaining uncertainty

During the original port verification, the game was not launched and no save was
loaded. The later toggle slice exercises a real textbox in a disposable session;
see its linked record above. At the end of the original port, dialogue rendering, controller
behavior, Steam update/reinstallation, other platforms, and mod conflicts have not
been exercised. Some robe trim shares the mapped colors. The blue study is a
pipeline test, not a finished complexion preset. Use manual region masks and art
review before expanding coverage.

Only source code, synthetic tests, configuration, and documentation are candidates
for version control. Game copies, extracted assets, packages, and previews remain
ignored. No game-derived material was staged or committed.
