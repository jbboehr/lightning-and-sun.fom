# First finish-plan integration batch

2026-11-05. Authoring, integration and the combined verification checkpoint are complete.
**New artwork acceptance remains pending.** The user requested one review after
this batch's implementation and verification. The previously displayed Elsie
work remains accepted; it is not being submitted again.

## Content and preserved artwork

| Candidate | New world strips | Native frames | West mirrors | Evidence |
| --- | ---: | ---: | ---: | --- |
| Louis, all seasons | 44 | 168 | 60 | [Material decisions and checks](louis-world.md) |
| Merri, all seasons | 44 | 168 | 60 | [Material decisions and checks](merri-world.md) |
| Adeline, Beach/Wedding | 33 | 110 | 44 | [Material decisions and checks](adeline-world-finish.md) |
| Juniper, Beach Accident | 6 | 31 | 4 | [Transformation/material decisions](juniper-beach-accident.md) |
| Total submitted | 127 | 477 | 168 | 645 complete comparison cases |

The [consolidated artwork review](../../tmp/finish-batch-review/index.html)
links every case and sampled five-choice overviews. Its
[candidate manifest](../../tmp/finish-batch-review/candidate-manifest.json)
fingerprints all four profiles, preset sets and complete galleries. No candidate
was written into an accepted fixture. The authors inspected all native and
mirrored cases; that is separate from the user's judgment.

Literal source landmarks and per-frame counts protect shared-color cuffs,
clothing, bangles, crown and trim. All four variants validate, and omission/spill
negative controls fail at the intended source pixels. The author checks compare
5,208 retained variant PNG/metadata files byte-for-byte. Current galleries bind
131,906,304 preview pixels to their actual bundles and include every visible
source pixel. Per-character evidence owns the detailed materials, no-skin
frames, measurements and browser checks.

Including the separately accepted six Elsie strips, 133 baseline omissions are
now covered. The collection selects 5,042 strips: 2,201 portraits and 2,841 world
strips, generating 20,168 nonvanilla variants for 36 characters. Twenty characters
have complete seasonal world coverage. Adeline has 315 selected regions, Juniper
311, Louis/Merri 76 each and Elsie 164. Registry and selected-profile membership
agree exactly. The unchanged baseline omission inventory now leaves 1,270
recolor-review strips, 30 audited shadows and 18 spectral no-change strips.
Named portrait corrections and later content waves remain open.

All artwork remains pinned to archive SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The user supplied `tmp/fom` with a Linux executable and a different archive,
SHA-256 `bb40a6a3b063e086cecd89e8862d6cd5930e41c593a8f2290f4e0ad48b0d117b`.
Every selected animation PNG and metadata member (10,084 members) is byte-identical
between those archives. This does not establish compatibility with every game
update or unchanged native rendering behavior.

## Palette controls and native engine checks

[Palette controls](palette-controls.md) now use the existing Settings screen,
36 independent character rows, reset and optional remembered choices. Vanilla
and remembering Off are the defaults. Storage uses stable IDs in the MMAPI
`lns_palette` namespace. Release packages register no character hotkeys. The
packager and installed-archive verifier include all three production GML files;
MOMI v0.16.4 is required. End-user instructions now describe the menu.

The single runtime reliability review found inaccessible trailing warnings.
The fix adds immediate native InfoToasts and a focusable status row; its regression
failed before the fix and passed afterward. An additional boundary test verifies
that an incompatible parsed primary does not silently load a valid stale backup.

The supplied executable ran a separate, disposable 36-character reduced package
in private Xvfb with Mesa llvmpipe. All five fresh-process phases passed in the
actual game: default/no JSON write, saving, restart restoration, opt-out, and
Vanilla after opt-out. Each phase also constructed the native Settings page,
checked eight categories and 39 palette pilot entries, invoked native callbacks,
reopened the page and checked labels, pilot and scroller identity. A sixth phase
made the mod file and directory read-only: failed opt-out preserved session blue,
left the prior saved remember setting intact, set the error state and successfully
called the native notification API with the warning that old choices may return.
Permissions and the disposable opt-out envelope were restored afterward.

These are native API/IO tests with direct callback dispatch, not physical input or
rendered acceptance. Both modded and unmodified supplied archives produced black
screenshots in the private display. The environment therefore does not establish
visible wrapping/notifications, focus/scroll usability, mouse/controller/keyboard
navigation, natural dialogue, room/save transitions or current artwork in play.
Those remain P6 gates. No real saves/settings or Steam library were exposed.
No local GPU or model inference was used for these software-rendered probes.

The lab-only driver is outside the production source/package tree, requires an
explicit `LNS_PROBE_MODE` and registers no callback otherwise. Production package
inputs contain only the three shipped scripts. Reproduction inputs and source
API evidence are under `local/authoring/finish-ui/`; native logs and primary/backup
JSON are under `tmp/finish-ui-live-state/`.

## Combined verification

Fresh combined checks passed: formatting, all-target Clippy with warnings denied,
422 regular tests (489 opt-in tests remain ignored in that invocation), release
build, five GML interpreter cases and fourteen affected material/package cases.
Exact runtime-table fixtures now include stable preset IDs while retaining the
prior columns and pixel assertions.

The full 36-character CLI installation generated all 20,168 variants, passed strict
MOMI compilation and independently verified installed atlas pixels, metadata and
all three production scripts. Uninstall succeeded, removed recovery state and
restored the exact original archive hash shown above. The supplied `tmp/fom`
archive also retains its original hash. No user game installation was modified.

The consolidated review passed desktop/mobile Chromium checks: all four samples
and every link load, with no horizontal overflow. Candidate fingerprints were
rechecked after integration. New artwork is still awaiting the user's acceptance.

Reproduction commands and logs:

- Normal fmt/Clippy/test/release checks, five GML interpreter cases and five
  affected material/package suites: `tmp/finish-batch-verified-checks.log`.
- Full 36-character generation, strict MOMI compile, installed atlas/script
  verification and exact removal: `tmp/finish-batch-install-roundtrip.log`.
- Source/member agreement and selected totals: `tmp/finish-batch-counts.json`.
- Native startup/restart phases: `tmp/finish-ui-live-state/native-phase-results.json`.
- Native permission failure: `tmp/finish-ui-live-state/native-extra-results.json`.

The original Adeline 143-asset identities and the preserved 282-strip action
profile continue to have their separate archive-review contract. The new 315-strip
finish profile additionally proves retention of all 282 earlier regions/outputs.
Historical pixel and accepted-fixture assertions remain intact.

## Local retention

The restored full-install lab is a 601 MiB task copy; the native UI/IO lab is
1.2 GiB and retains its guarded QA driver for reproduction. Source archives and
accepted historical fixtures are preserved. The useful shared Rust cache is
34 GiB. Temporary Nix graphics-tool roots are retained for the remaining P6 work;
removing them now would repeat dependency downloads. No active test game remains.
