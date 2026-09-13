# Adeline standard Spring overworld actions

This completes the 25 standard strips in Adeline's Spring sprite directory.
The eight additions are general actions in three directions, shocked
start/loop/end, sleep, and kiss. They contain 29 frame occurrences and 19 distinct
images. Together with the earlier 17 overworld strips and 126 portraits, the
existing `adeline-world-actions-trial.json` set now covers 151 source strips and
604 generated variants. F6 keeps portraits and supported world sprites on the
same choice: Vanilla, Debug Blue, Hayden, Ryis, or Seridia.

The combined 36-character collection contains 2,226 sources: 2,201 portrait strips
and 25 Adeline overworld strips, producing 8,904 variants. Shared Rust and GML
runtime code is unchanged; this extends the reviewed profile and registry.

## Sources and masks

The read-only source is `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The new paths use
`assets/animations/NPCs/Adeline/Sprites/Spring/spr_npc_adeline_spring_`.
Every frame is 80×80, on the Default atlas, with Middle/54 origin.

| Suffix | Frames | Skin pixels changed per target |
| --- | ---: | ---: |
| `action_north` | 7 | 56 |
| `action_south` | 7 | 344 |
| `action_east` | 7 | 306 |
| `shocked_start_south` | 1 | 64 |
| `shocked_loop_south` | 1 | 72 |
| `shocked_end_south` | 1 | 64 |
| `sleep_east` | 1 | 47 |
| `kiss_east` | 4 | 197 |

General-action durations are `[0.1,0.25,0.25,0.25,0.25,0.1,0.4]`;
kiss durations are `[0.15,0.15,0.8,0.15]`. The single-frame strips omit
`frame_len` and `duration`. All source metadata is preserved. Native NPC data
defines shocked as a complex start/loop/end animation and general action with
a randomized final-frame hold of 240–360. The original animator owns those
transitions and holds. West uses the east artwork mirrored for action, sleep,
and kiss. Kiss hearts are separate game effects, outside these sprite masks.

`palettes/profiles/adeline-world-actions.json` adds 107 seeds for a total of
4,293. The existing four-color world ramp remains sufficient; palette sets,
color groups and target colors are unchanged. Each target adds 1,150 recolored
pixels while protecting 156 matching material pixels. The complete Adeline set
changes 424,698 pixels per target. Profile SHA-256:
`bd08641643feb4f8d4eec78cf031be311972d2c29d541925a184c036aaf314fb`.

Skin selections include tiny rear-facing hands, deep finger creases, the
isolated far hand in the final east action frame, and the kiss face edge.
Shared-color cape, gold and trouser details stay original, as do hair, eyes and
mouth interiors. The author inspected all 29 occurrences in all four targets.
Independent review inspected all 19 distinct images in all five choices and
checked 76 literal source-art landmarks. No concrete art defects remained.

All 143 earlier regions are unchanged. The combined build retains 1,430 earlier
Adeline original/variant PNG and metadata files exactly, and adds 80 files for
the eight strips and their four variants. The other 35 character trees are
byte-identical to the Seridia batch (20,925 files). All 604 Adeline variants and
their metadata match the reviewed author outputs. Evidence:
`generated/adeline-standard-author/final-audit.json`,
`tmp/adeline-standard-art-review.md`, and
`tmp/adeline-standard-comparison.log`.

## Offline review

- [Five-palette summary](../../../generated/adeline-standard-preview/summary.png):
  explicitly a sample, showing general-action South frame 4 of 7.
- [Complete Vanilla/Debug Blue review](../../../generated/adeline-standard-preview/blue-review/index.html):
  all 29 source frames and 12 mirrored West views, on six small pages.

The 41 direction/frame cases give 82 Vanilla/Blue views, with full sprites
enlarged at 10× nearest-neighbor. Exact comparison checks all 8,610,000 placed
review pixels and 525,000 summary pixels, including horizontal reversal and
absence of clipping. Chromium decoded all 41 sheets and checked every page,
link, anchor and expected case. The summary and representative layouts were
visually inspected. The complete preview tree is 892,608 bytes.
The user accepted the offline previews; individual reviewed cases were not recorded.
Evidence: `tmp/adeline-standard-preview-pixel-check.log` and
`tmp/adeline-standard-preview-browser-check.json`.

## Verification

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test world_actions --test overworld -- --ignored --nocapture'
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" cargo test --locked --test gml_runtime -- --ignored'
```

Formatting, Clippy, 87 active tests, the release build, three local world corpus
tests and three GML tests passed. The package fixture first failed on unsupported
`action_east`, then passed after the eight exact registry entries were added.
It checks durations, missing single-frame timing defaults, offsets, fresh IDs,
PNG bytes and the complete generated table; invented directions remain rejected.

The new local corpus test covers all four targets and all 29 frames, alpha,
metadata, exact target colors, equal mask selections and 28 literal landmarks
(21 skin and seven protected). Removing the isolated `[525,46]` far-hand seed
in an ignored copy made it fail at that exact east-action pixel; the unchanged
production profile passed again. Logs: `tmp/adeline-standard-checks.log`,
`tmp/adeline-standard-package-{red,green}.log`, and
`tmp/adeline-standard-missing-hand-{red,green}.log`.

The focused runtime probe executes the original game's `NpcAnimationHandler`
and NPC `animate` method, the shipped palette wrapper, and the actual generated
Adeline and Hayden table rows. It passed 41 direction/frame cases, 205 palette
observations and 20 shocked transitions. It checks fractional animation state,
original sprite/pack identity, loop counters, West flipping, paused updates,
the general-action hold, portrait synchronization and independent Hayden choices.
Asset IDs, instance lookup, UI and sprite metadata access are simulated; metadata
durations are used in relative units. This verifies state transitions rather
than real wall-clock playback timing. Evidence:
`tmp/adeline-standard-runtime-inputs.json` and
`tmp/adeline-standard-runtime-root.log`.

The combined build and all 8,904 exact variant validations passed. A fresh
MOMI installation in `tmp/adeline-standard-playtest` verified installed pixels,
animation metadata and the generated table. Installed archive SHA-256:
`f1b70b1e658d77362d5bd70eae9ba5b0ecfbc5fbc7f736228cb6ea798dadfe89`.
Both the source backup and retained `previous.zip` still match the original
source hash. See `tmp/adeline-standard-all-variants.log`,
`tmp/adeline-standard-install-report.json`, and
`tmp/adeline-standard-source-after.sha256`. Recipe/registry inputs still match
`tmp/adeline-standard-frozen-inputs.sha256`.

The mounted game directory was empty. Native gameplay, natural scheduling,
furniture/story interactions and an uninstall roundtrip were not exercised.
The isolated player package has no preview helper; the existing desktop
launcher stays on its earlier Zorel copy. All images, archives, helpers and
local game data remain ignored by Git.

## Remaining coverage

Seventeen special-animation Spring strips remain: writing, reading, working,
thinking, finger snapping and fainting. Other overworld outfits and other
characters' world sprites remain original. See the
[preceding action record](overworld-actions.md) for the existing world hook and
its earlier native playtest coverage.
