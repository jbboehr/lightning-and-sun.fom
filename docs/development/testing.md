# Stable local test corpora

World-sprite tests for Balor, Valen, Eiland, Hayden, Ryis, Reina, Juniper, Celine
and March read `extracted/test-corpus/<character>/`. This is an ignored local
directory or symlink containing the character's complete current source corpus,
including portraits and animation metadata. It is not an accepted output baseline.

After exporting a fresh corpus for a slice, point the character's stable path
at that export before running its local tests. For example, from the repository
root on Linux:

```sh
mkdir -p extracted/test-corpus
ln -sfnT ../balor-winter-finish-study extracted/test-corpus/balor
nix-shell --pure --run 'cargo test --release --locked --test balor_winter_finish -- --ignored'
```

Use the freshly exported study directory for each character being expanded.
The target must contain every asset in that character's current profile, with
its original metadata. Existing hash checks still reject changed source PNGs;
the stable path does not bypass them. A missing corpus is a setup error, not a
reason to skip an explicitly requested local test. No game assets are tracked.
Release-mode tests use the optimized CLI, which makes repeated full-corpus PNG
generation substantially faster.

## Adding coverage

1. Extend the character profile and animation registry, and add the new slice's
   material/pixel and package tests.
2. Update that character's count in `tests/profile_coverage.rs`. This regular
   test checks all characters' totals, duplicate assets, and exact agreement
   between the selected profiles and the animation registry without game files.
3. Refresh the local corpus link and run the new and retained material tests.
   New tests use the same stable path. Do not add the growing profile total to
   their assertions or ignore descriptions.

Historical tests keep their literal sprite lists, per-frame pixel expectations,
fixed slice boundaries, and accepted output paths. Never point their baseline
at `generated/build` or update it to the candidate output. Those comparisons
must continue checking against the previously accepted artwork. Tests may still
build the full current profile; the stable corpus supplies its inputs without
requiring edits to historical tests.

This migration covers the nine cumulative world corpora above. Older Adeline
experiments and portrait-only tests retain their separate fixtures.

## Normal checks

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
```

Local artwork tests remain explicit opt-ins because their source images and
accepted generated baselines are intentionally absent from Git. Use `--test`
to select the affected targets, followed by `-- --ignored`; do not run every
ignored test indiscriminately, since some exercise local installation tools.

## Migration verification

On 2026-09-30, formatting, Clippy with warnings denied, all 214 regular tests,
all 216 migrated local artwork tests (in release mode), and the release build
passed. The migration moved 79 repeated total-count assertions into the central
check. A mechanical comparison confirmed that every other assertion and accepted
baseline path in those 216 tests stayed unchanged.

An isolated copy reproduced the old Balor Autumn test's count failure with 228
sources. The migrated test passed with both 228 and 229 sources without editing
the test. The central check separately rejected a reduced total, duplicate asset,
and registry mismatch. Six deliberate skin-omission/material-spill variants still
failed at their intended pixel assertions. Evidence is local under
`tmp/test-churn-*`. Installation and live gameplay were not rerun for this test-only
change.
