# Adeline Spring special animations

This completes all 42 PNG strips in Adeline's Spring overworld directory in the
reviewed archive. The 17 additions cover standing and seated writing, seated
reading and working, thinking, finger snapping, and fainting. They contain
52 frame occurrences and 43 distinct images. Together with the
[25 standard strips](overworld-standard-actions.md) and 126 portraits, the
existing `adeline-world-actions-trial.json` set covers 168 sources and produces
672 variants. F6 keeps portraits and supported world sprites on the same choice:
Vanilla, Debug Blue, Hayden, Ryis, or Seridia.

The combined 36-character collection now contains 2,243 sources: 2,201 portrait
strips and 42 Adeline overworld strips, producing 8,972 variants. This extends
the existing profile and registry; shared Rust and GML runtime code is unchanged.

## Sources and masks

The read-only source is `tmp/momi-lab/assets.bak.zip`, SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
New paths use
`assets/animations/NPCs/Adeline/Sprites/Spring/spr_npc_adeline_specialanimation_spring_`.
All are South-facing, 80×80 frames on the Default atlas, with Middle/54 origin.
The source archive and reviewed profile have exactly the same 42 Spring PNG
paths. No additional mirrored directions are defined for these special cycles.

| Cycle | Strips | Frames | Skin pixels changed per target |
| --- | ---: | ---: | ---: |
| `write` start/loop/end | 3 | 8 | 376 |
| `write_sit` start/loop/end | 3 | 8 | 379 |
| `read_sit` start/loop/end | 3 | 10 | 380 |
| `work_sit` start/loop/end | 3 | 10 | 520 |
| `think` start/loop/end | 3 | 4 | 238 |
| `finger_snap` | 1 | 5 | 321 |
| `faint` | 1 | 7 | 443 |

Source metadata is preserved, including variable-duration lists and omitted
single-frame timing fields. The five start/loop/end families use the native
complex animator; writing, reading and working while seated retain their native
seated flag. Finger snapping and fainting are linear. The original game still
owns animation progression, pause behavior, furniture use and scheduling.

`palettes/profiles/adeline-world-actions.json` adds 255 seeds, for 4,548 total.
No new source shades or target colors were needed. Each target adds 2,657
recolored pixels while protecting 110 matching clothing pixels. The complete
Adeline set changes 427,355 pixels per target. Profile SHA-256:
`5d743aa5ad5ed788886189e2c461d2c3cf842152455b73217f6222ab306f35a0`.

Masks include fingers below writing boards, book-edge fingertips, raised
thinking hands, later working frames and the lowered fainting pose. The
isolated two-pixel hand continuation in finger-snap frames 1–3 recolors; adjacent
chest trim stays original. Boards, pens, pages, book covers, hair, clothing,
sparkles, sweat and mouth effects retain their original colors.

The author and independent art reviewer inspected all 43 distinct frames in
Vanilla and all four targets; exact comparisons cover the nine repeated frame
occurrences. All 58 independent skin/material landmarks passed. Evidence:
`generated/adeline-special-author/final-audit.json` and
`tmp/adeline-special-art-review.md`.

The earlier 151 regions are unchanged. The combined build preserves 1,510
earlier Adeline original/variant PNG and metadata files, and adds 170 files.
The other 35 character trees are byte-identical to the preceding standard-action
bundle (20,925 files). All 672 Adeline variants and metadata match the reviewed
author outputs. See `tmp/adeline-special-comparison.log`.

## Offline review

- [Five-palette summary](../../generated/adeline-special-preview/summary.png):
  a sample showing standing writing, loop frame 2 of 4.
- [Complete Vanilla/Debug Blue review](../../generated/adeline-special-preview/blue-review/index.html):
  all 52 South-facing frames and 104 views on nine pages, at most eight cases
  per page. All frames are static and enlarged at 10× nearest-neighbor.

The full preview tree is 1,134,838 bytes. Exact comparison checks 10,608,000
placed review pixels and 510,000 summary pixels. Fresh bounds include all props
and lowered poses without clipping. Chromium decoded all 52 sheets and checked
every page, link, anchor and expected case. The summary and representative
layouts, including the final fainting frame, were visually inspected. Evidence:
`tmp/adeline-special-preview-pixel-check.log` and
`tmp/adeline-special-preview-browser-check.json`.
The user accepted the offline previews; individual reviewed cases were not recorded.

## Verification

```sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --locked --all-targets -- -D warnings && cargo test --locked && cargo build --locked --release'
nix-shell --pure --run 'cargo test --locked --test world_actions --test overworld -- --ignored --nocapture'
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/tmp/runtime-capabilities/fabricator/target/debug/interpreter" cargo test --locked --test gml_runtime -- --ignored'
```

Formatting, Clippy, 87 active tests, the release build, three local world corpus
tests and three GML tests passed. The expanded package fixture first failed on
unsupported `specialanimation_spring_faint_south`, then passed after the exact
registry entries were added. It checks original durations, omitted single-frame
timing fields, offsets, fresh IDs, PNG bytes and the complete generated table;
invented directions remain rejected.

The extended material-boundary corpus test checks all four targets and all
81 standard/special frame occurrences in its 25 strips, preserving the preceding
28 landmarks and adding 20 new ones. It checks alpha, metadata, exact target
colors and identical selection across targets. Removing the isolated finger-snap
hand seed in an ignored copy makes it fail at `[117,47]`, while the production
profile passes. The other two local tests retain idle/walk and everyday-action
coverage, bringing the Spring corpus total to 42 strips and 127 frames. Logs:
`tmp/adeline-special-checks.log`, `tmp/adeline-special-package-{red,green}.log`,
and `tmp/adeline-special-missing-hand-{red,green}.log`.

The focused runtime probe executes the original game's `NpcAnimationHandler`
and NPC `animate` method with the shipped palette wrapper and actual generated
Adeline/Hayden table rows. It passed 52 South-facing frame cases and 260 palette
observations, 100 transitions across the five complex families at their actual
last frames, and 20 finger-snap/faint completions with and without a held final
frame. It preserves seated flags, source sprite and pack identity, animation
state, fractional phase, position, paused refresh, portrait phase and independent
Hayden choices. Engine asset IDs, instance lookup, UI and metadata access are
simulated, with original durations in relative units. The full NPC factory/state
machine, pause-policy switching and live playback timing are outside this probe.
Evidence: `tmp/adeline-special-runtime-inputs.json` and
`tmp/adeline-special-runtime-root.log`.

The combined build and all 8,972 exact variant validations passed. A fresh MOMI
installation in `tmp/adeline-special-playtest` verified installed pixels,
animation metadata and the generated table. Installed archive SHA-256:
`fcef20e43625776f0a61b4db61baf93db2d445759c4e725a34b2d6b812450c91`.
Both the source backup and retained `previous.zip` still match the original
source hash. See `tmp/adeline-special-all-variants.log`,
`tmp/adeline-special-install-report.json`, and
`tmp/adeline-special-source-after.sha256`. Recipe/registry inputs are recorded in
`tmp/adeline-special-frozen-inputs.sha256`.

The mounted game directory is empty. Native gameplay, natural scheduling,
furniture/story interactions and an uninstall roundtrip were not exercised.
The isolated package has no preview helper; the desktop launcher remains on its
earlier Zorel copy. All images, archives, helpers and local game data stay ignored.

## Remaining coverage

Adeline's other overworld outfits and other characters' overworld sprites remain
original. All Adeline portraits and all Spring overworld strips in the reviewed
archive now have masks. The accepted offline review covers recolored art;
interactive gameplay and native playback timing remain unverified.
