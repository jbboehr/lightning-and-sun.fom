# Explicit source PNG hash override

`--allow-source-hash-mismatch` lets a user try existing region masks against
changed PNG bytes without editing their tracked hashes. It is supported by
`apply`, `build-presets`, `build-characters`, `install` and `validate --palette`.
The validation command requires a palette when the flag is present.
`review-batch` continues to require matching reference hashes.

The flag is passed explicitly through generation to the mask's hash comparison.
There is no persistent setting or recipe mutation. A differing hash prints the
asset filename, expected SHA-256 and actual SHA-256 to stderr. JSON reports still
record the actual source hash. Matching hashes produce no warning.

Only hash equality is relaxed. Recipe parsing, exact inventory, dimensions,
seed bounds and source colors, transparency, exact output validation, native
metadata and installer publication checks retain their existing behavior.
An old seed may still reach the wrong material in changed artwork; successful
validation does not establish that the recoloring looks correct.

## Verification

The new synthetic CLI regression first failed because the flag was unrecognized.
It now exercises genuinely changed PNG bytes through all three generation paths,
checks selected and excluded pixels, verifies actual hashes in JSON, and confirms
that profile/input bytes and a subsequent strict invocation remain unchanged.
Boundary cases reject changed dimensions, invalid seeds, wrong inventory,
malformed hashes, wrong output pixels and alpha changes.

The installer regression covers palette, presets and character inputs. Each
rejects a stale hash by default, rejects wrong installed atlas pixels even with
the flag, then installs and restores the original archive with valid output.
Those tests use a synthetic runner and atlas; no new live-game installation was
performed for this flag.

An additional default-palette regression supplies a changed PNG with matching
dimensions and valid source-color seeds. Strict mode stops at the stored hash;
the override reaches generation and then rejects deliberately mismatched
installed artwork, leaving the game archive and recovery state untouched.

The release binary also generated Ryis's 233 existing PNGs with the flag. All
466 PNG/metadata files were byte-identical to the reviewed Debug Blue baseline;
no mismatch warnings appeared, and exact palette validation passed. All eleven
frozen inputs from the preceding art slice remain unchanged.

Evidence: `tmp/source-hash-override-{red,targeted,checks}.log`,
`tmp/source-hash-override-ryis-{comparison.log,validation.json}` and
`tmp/source-hash-override-profiles-unchanged.log`.

## Reliability review

Verdict: **PASS_WITH_RESIDUAL_RISK**. Independent Breaker and Test Attacker passes
found no demonstrated defects or accepted static findings. The Test Attacker
added the default-palette installer regression above; it and the complete
installer/override test suites pass. No production fixes were needed.

Final verification passed formatting, Clippy, all 113 active tests (104 local
opt-in tests ignored) and the release build with:

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
```

The final log is `tmp/source-hash-override-final-checks.log`. Reviewed production
file hashes remain unchanged after the independent passes; the final diff has
no whitespace errors.

The remaining limitation is visual suitability of existing masks for changed
artwork. The override deliberately permits that experiment; neither matching
geometry nor passing exact-color checks proves that the mask still selects skin.
A successful default-mode override against real MOMI was not rerun.
