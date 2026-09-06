# CLI installer prototype

This slice makes the existing one-portrait experiment repeatable. It does not
expand recoloring coverage: region masks, all 25 Adeline spring portrait strips,
then other outfits and overworld animations remain the next art/pipeline slices.

## Target and dependency handling

The tested MOMI v0.15.10 CLI has no install-directory override. Its locator checks
Steam before the current directory. `shell.nix` therefore provides a runner that
only exposes the build directory as `/game` inside bubblewrap. Neither the user's
Steam installation nor their saves are mounted. Nix fetches MOMI with SHA-256
`a0a068c4f0b8f4f85d801fc2aa2b6611f0e172c0d7a629cbec5713d6ab235a12` and supplies its
loader and libraries. The Rust binary embeds the runner/default tool paths when
built in this shell; these remain Nix store dependencies, not portable bundled
executables. The installer is limited to x86_64 Linux. The other palette commands
retain their existing platform scope.

MOMI runs with strict lints, fail-on-skip, and required compilation checks. Its
CLI can nevertheless skip an invalid content mod before that stage and return
success. The wrapper independently compares its selected folder manifests with
MOMI's generated `config/mods/manifest.json`, including names, versions, and count.
For a previously modified archive, `--installed-mods` must supply the current
config-side MOMI selection. The copied folders must match that selection before
adding the palette. A nonempty mod directory alone cannot establish that every
installed mod's source folder remains available. Unsupported layouts and selection
mismatches stop before publication.

Selection checks compare sorted copies while retaining the supplied and rebuilt
manifest order. Before publication, the rebuilt list restricted to existing mods
must match their prior order exactly. MOMI's CLI orders by mod name; a GUI-defined
custom order that the CLI cannot reproduce therefore stops installation. The
palette's position in the rebuilt list does not affect this comparison. No change
to the MOMI runner or custom-order support is required for this refusal behavior.

MOMI's archive marker is empty and its config-side list carries no archive hash.
The caller must provide the list associated with the current archive. The wrapper
cannot detect a stale list or recover missing mod sources. It reads the supplied
list without writing into the real config/save directory.

## Installation and recovery

An advisory file lock serializes this CLI's operations on a canonical game path.
The game and unrelated installers must be closed. Archive and mod symlinks are
rejected. Work files live in a fresh temporary directory on the game's filesystem.

The wrapper copies the live archive as `previous.zip`, chooses vanilla from the
live archive or the existing MOMI backup, copies the current mod source folders,
then calls the existing Rust export/apply/package pipeline. It invokes MOMI only
against those copies. The installed selection and exact vanilla/blue atlas pixels,
animation properties, and GML bytes must pass verification. MOMI does not emit a
loose variant PNG; validation reads its atlas placements instead.

After checking that the live archive has not changed during the build, the wrapper
synchronizes its backup, prepared archive, and checksum receipt. It publishes
`.mistria-palette/` before atomically renaming the prepared ZIP over `assets.zip`.
An interrupted operation can therefore leave either a checked installation or a
prepared receipt beside the unchanged previous archive. `uninstall` handles both.
The existing `assets.bak.zip`, source mod folders, executable, and saves are not
modified. Build intermediates are discarded; the previous archive remains local.

Uninstall checks the receipt against the current archive before restoring the
backup through another rename. An external change or damaged recovery archive
stops restoration. If restoration already happened, uninstall can finish cleaning
the recovery files. Unknown files inside the state directory are not deleted.
Repeated install requires uninstall first, allowing a changed recipe to be tested
without accidentally replacing the original recovery snapshot.

This wrapper does not register its generated package in the live `mods/` folder
or write into game config directories. Uninstall it before using MOMI separately.
Its snapshot restoration preserves the exact modded archive that preceded it;
external mod changes made afterward must be reconciled separately.

## Verification commands

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
```

Synthetic installer tests exercise the real CLI and a small external-runner
fixture. They cover selected-mod preservation, exact restore, incomplete output,
silent mod skipping, damaged recovery data, prepared-but-unpublished recovery,
and refusal to overwrite a game update. Independent corruptions of atlas pixels,
variant metadata, and the game script must each prevent publication. The loose-PNG
assumption and silent-skip check both had failing reproductions before correction.

The independent review and test pass reproduced two additional failures: a marked
archive could lose a previously installed mod when its source folder was missing,
and an interruption after deleting the recovery receipt could leave an empty
directory that blocked further commands. The regression tests require a supplied
prior selection to match the source folders and allow empty-state cleanup, while
retaining nonempty state with unknown files or a missing receipt. Nested folder
manifests remain explicitly unsupported; that review suggestion did not expand
the installer scope.

For a real archive round trip, use a fresh local copy; do not point a development
test at the supplied original or a live Steam install:

```sh
mkdir tmp/cli-installer-lab
cp tmp/fields-of-mistria/assets.zip tmp/fields-of-mistria/Maybe.toml tmp/cli-installer-lab/
target/release/mistria-palette install --game-dir tmp/cli-installer-lab
target/release/mistria-palette uninstall --game-dir tmp/cli-installer-lab
cmp tmp/fields-of-mistria/assets.zip tmp/cli-installer-lab/assets.zip
```

The source archive fingerprint and the prior in-game F6/animation verification are
recorded in [portrait-toggle.md](portrait-toggle.md). This slice changes how that
package is installed, not the game script. It does not repeat the full live-game
smoke test. Config-side Mods-tab integration, concurrent external installers,
automatic recovery after game updates, and other platforms remain outside scope.

## Local integration evidence

On 2026-09-06, the default Nix-built binary completed install/remove against a
fresh copy of the supplied archive. Installed pixels passed the production atlas
verifier. Removal matched the source archive byte-for-byte.

A second run first installed an independently authored GML probe with MOMI, kept
its source folder in the lab, then installed the palette through this CLI. Both
scripts were present in the resulting archive. CLI removal restored the exact
pre-palette archive containing the probe, and MOMI's existing pristine backup
still matched the supplied source. The lab is left in that pre-palette state.
Reports and archive snapshots remain in ignored `tmp/cli-*` paths.

After the review fixes, that coexistence round trip passed again with the current
MOMI selection supplied explicitly:

```sh
target/release/mistria-palette install --game-dir tmp/cli-installer-lab \
  --installed-mods tmp/cli-installer-lab/config/mods/manifest.json
target/release/mistria-palette uninstall --game-dir tmp/cli-installer-lab
cmp tmp/cli-before-othermod.zip tmp/cli-installer-lab/assets.zip
cmp tmp/fields-of-mistria/assets.zip tmp/cli-installer-lab/assets.bak.zip
```

All commands exited zero. The post-removal archive hash was
`7a6f08bf68c931efbcc374aba941ae0b55d9d2f627cb1d7e50efeb85432010ca`, matching the
probe-only snapshot. The installed variant retained its expected PNG hash in the
generation report and passed the production atlas verifier.

Both Nix shell definitions evaluated with `nix flake check --no-build --all-systems`.
Execution was checked on x86_64 Linux only.

Final reliability verdict: **PASS_WITH_RESIDUAL_RISK** for this CLI prototype.
The full verification command above passed formatting, Clippy with warnings denied,
all 34 active synthetic tests (including 15 installer tests), and the release build.
The focused installer suite also passed through `nix develop`. The normal suite's
three optional game/interpreter checks remained ignored; this slice instead ran
the real CLI integration described above. No live-game smoke test was repeated.
Current-list provenance, concurrent external installers, other platforms, and wider
recoloring coverage remain the concrete limits described above. Game and derived
files remain ignored.

## Load-order review follow-up

The supplied review identified that sorting both installed-mod lists discarded
custom load order. An independent review of the wrapper and pinned MOMI source
confirmed that the GUI can supply an order while the CLI sorts by name, and that
the generated manifest records the actual installation order. The finding was
accepted. Sorted copies still validate membership; a separate comparison now
rejects a rebuilt archive that changes existing mod precedence.

`rebuilding_must_preserve_existing_mod_order` failed before the fix because the
CLI published a reversed existing-mod order. It now covers both mismatch
directions, matching alphabetical and custom orders, and a palette inserted
between existing mods. Failure must retain the exact live archive, MOMI backup,
and config manifest without publishing recovery state.

A disposable real-runner fixture used two original `.mist` probes that overwrite
the same unused asset. Its initial archive/list represented Zulu then Alpha,
with Alpha winning. The pre-fix binary succeeded but MOMI's alphabetical rebuild
changed the winner to Zulu. After restoring the snapshot, the fixed binary exited
1 with the load-order error; byte comparisons confirmed the prior archive and
pristine backup were unchanged and no palette state was published. No GUI session
was run: the custom-order starting archive was constructed as a fixture. Evidence
is kept under ignored `tmp/load-order-*` paths.

For the success path, the real MOMI runner first rebuilt the two content probes
in its normal Alpha/Zulu order and generated the corresponding config list. The
fixed palette installer then succeeded, kept Zulu's override, and installed the
palette script. Removal restored that pre-palette archive byte-for-byte. The lab
is left with those two probes and no palette installation; the supplied read-only
game archive retains its recorded SHA-256.

The focused independent code review found no further defects. The independent
test pass expanded coverage to every palette position and to extra/duplicate
entries in the supplied list, with no additional failures. Final formatting,
Clippy, all 34 active tests, and the release build passed using the full command
above. The three opt-in checks and live-game behavior were not rerun. The
reliability verdict remains **PASS_WITH_RESIDUAL_RISK** within these stated limits.
