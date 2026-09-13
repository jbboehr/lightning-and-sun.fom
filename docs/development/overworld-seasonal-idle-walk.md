# Adeline seasonal idle and walking animations

This adds the 18 idle/walk strips from Adeline's Summer, Autumn, and Winter
overworld outfits. They contain 45 source frame occurrences and 28 distinct
images. Together with the [complete Spring coverage](overworld-special-actions.md)
and 126 portraits, the existing `adeline-world-actions-trial.json` set covers
186 sources and produces 744 variants. F6 keeps portraits and supported world
sprites on the same choice: Vanilla, Debug Blue, Hayden, Ryis, or Seridia.

The combined 36-character collection contains 2,261 sources: 2,201 portrait
strips and 60 Adeline overworld strips, producing 9,044 variants. Idle and walking
now have masks in all four seasonal outfits. Shared Rust and GML runtime code
is unchanged.

## Sources and masks

The read-only source is `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
New paths use
`assets/animations/NPCs/Adeline/Sprites/{Summer,Autumn,Winter}/spr_npc_adeline_{summer,autumn,winter}_{idle,walk}_{north,south,east}.png`.
Each outfit has six strips and 15 frames. The game mirrors East artwork for
West-facing movement; no separate West strip is registered.

All 18 original sidecars specify 80×80 frames on the Default atlas with Middle/54
origin. Idle omits frame count and duration; walking has four frames and duration
0.15. Source metadata is preserved. Native animation packs still own facing,
frame progression and movement; palette changes select the matching render
sprite through the existing getter wrapper.

Summer includes isolated far-arm pixels. Autumn and Winter retain exposed hands,
including Winter's rear-facing hands below the white cuffs. Their coat, boot and
collar details require separate material boundaries from the exposed skin.

The profile adds 187 seeds, for 4,735 total, without changing the source colors,
target palettes or prior 168 region definitions. It now separates the existing
portrait and world ramps into two `color_groups`. Autumn/Winter collar trim uses
portrait shade `#C47054` and touches world-ramp neck/face pixels; the split keeps
those materials out of skin components. Twenty connected trim pixels would
otherwise recolor. This uses the existing recipe format and does not change the
palette-processing code.

| Outfit | Strips | Frames | Skin pixels changed per target |
| --- | ---: | ---: | ---: |
| Summer | 6 | 15 | 528 |
| Autumn | 6 | 15 | 462 |
| Winter | 6 | 15 | 422 |

Each target adds 1,412 recolored pixels while protecting 284 matching material
pixels: 103 `#BA6A4C` and 181 `#C47054`. The complete Adeline set changes 428,767
pixels per target. Profile SHA-256:
`d93b523efa22d55b68182bc98d9928a59681cc5788291f7195d741635d2553ee`.
All 1,344 earlier Adeline variant PNG/metadata files remain byte-identical under
the grouping split. The author and independent art reviewer inspected all
28 distinct frames in Vanilla and all four targets, with exact comparisons for
repeated frame occurrences. All 50 independent skin/material landmarks passed.
Evidence: `generated/adeline-seasonal-world-author/final-audit.json` and
`tmp/adeline-seasonal-world-art-review.md`.

The final combined bundle preserves all 1,680 prior Adeline original/variant
PNG and metadata files and adds 180 files. All 744 Adeline variants and sidecars
match the reviewed author outputs. The other 35 character trees are byte-identical
to the preceding Spring bundle (20,925 files), and existing generated table rows,
palette choices and controls remain unchanged. See
`tmp/adeline-seasonal-world-comparison.log`.

## Offline review

- [Five-palette summary](../../generated/adeline-seasonal-world-preview/summary.png):
  one representative row per outfit.
- [Complete Vanilla/Debug Blue review](../../generated/adeline-seasonal-world-preview/blue-review/index.html):
  all 45 source frames and 15 mirrored West views, for 60 cases and 120 views.
  Each season has an Idle page, a North/South walking page and an East/West walking
  page, with at most eight cases per page.

The 1020×856 summary shows South-facing idle in all three outfits and all five
choices at 6×. The complete review uses 10× nearest-neighbor enlargement. The
whole preview tree is 1,347,278 bytes. Exact checks compare 12,240,000 placed
review pixels and 550,800 summary pixels, including West reversal and fresh
bounds without clipping. Chromium decoded all 60 sheets and checked every page,
link, anchor and case. The summary and representative layouts, including Winter
hands and the last West walking frame, were visually inspected. Evidence:
`tmp/adeline-seasonal-world-preview-pixel-check.log` and
`tmp/adeline-seasonal-world-preview-browser-check.json`.
The user accepted the offline previews; individual reviewed cases were not recorded.

## Verification

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test world_actions --test overworld -- --ignored --nocapture'
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" cargo test --locked --test gml_runtime -- --ignored'
```

Formatting, Clippy, 87 active tests, the release build, three local world corpus
tests and three GML tests passed. The package fixture now covers all 24 seasonal
idle/walk strips. It first failed on unsupported `autumn_idle_east`, then passed
after the 18 exact registry entries were added. It checks idle defaults, walking
timing, offsets, fresh IDs, PNG bytes and the complete generated table. Separate
checks reject invented West strips and the still-unreviewed Summer blink.

The extended material-boundary test checks all four targets and all 126 frame
occurrences in its 43 strips, retaining the preceding 48 landmarks and adding
20 seasonal hand, collar, cuff and clothing landmarks. It checks alpha, metadata,
exact target colors and identical selection across targets. An ignored candidate
copy without the color-group split fails the source-color guard on material
spill; the final grouped profile passes. Together with the other two local world
tests, the corpus covers 60 strips and 172 frames. Logs:
`tmp/adeline-seasonal-world-checks.log`,
`tmp/adeline-seasonal-world-package-{red,green}.log`, and
`tmp/adeline-seasonal-world-grouping-red.log`.

The focused runtime probe executes the original native `NpcAnimationHandler`
and NPC `animate` method with the shipped palette wrapper and actual generated
Adeline/Hayden table rows. It passed 60 direction/frame cases and 300 palette
observations, 30 outfit changes with 150 further observations, and 30 native
last-frame loop boundaries. Checks preserve source sprite/pack identity,
fractional and paused animation state, facing, West flips, seasonal portrait
synchronization and independent Hayden choices. Engine IDs, instance lookup,
UI and metadata access are simulated; original durations use relative units.
This does not exercise the full NPC state machine, calendar changes, natural
scheduling or real wall-clock timing. Evidence:
`tmp/adeline-seasonal-world-runtime-inputs.json` and
`tmp/adeline-seasonal-world-runtime-root.log`.

The combined build and all 9,044 exact variant validations passed. Evidence:
`tmp/adeline-seasonal-world-build-report.json` and
`tmp/adeline-seasonal-world-all-variants.log`. Recipe/registry inputs are bound in
`tmp/adeline-seasonal-world-frozen-inputs.sha256`.

A fresh MOMI installation in `tmp/adeline-seasonal-world-playtest` verified
installed pixels, animation metadata and the generated table. Installed archive
SHA-256: `a0e8d6f74fb9d5a1621086b9e35f40b0c70bb19b924e78a137b24b61ab70ecad`.
The source backup and retained `previous.zip` both still match the original
source hash. See `tmp/adeline-seasonal-world-install-report.json` and
`tmp/adeline-seasonal-world-source-after.sha256`.

Native gameplay, natural outfit transitions and an uninstall roundtrip were not
exercised. The isolated package has no preview helper; the desktop launcher
remains on its earlier Zorel copy. All images, archives, helpers and local game
data stay ignored by Git.

## Remaining coverage

Adeline's other Summer, Autumn, and Winter actions and other characters'
overworld sprites remain original. All Adeline portraits, all Spring overworld
strips, and idle/walk in the other three seasons now have masks. The accepted
offline review covers recolored art; native gameplay timing and natural outfit
transitions remain unverified.
