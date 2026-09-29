# Juniper Beach completion

This slice adds the remaining special swimming strips: `swim_idle_east`,
`swim_idle_north`, `swim_idle_south` and `swim_spell_cast_south`. Each has four
frames, giving sixteen source frames and four native West mirrors: twenty
review cases. Juniper now covers all 18 current Beach strips, with 290 total
sources and 1,160 variants. All 286 earlier region objects, pins, seventeen
source colors, nine groups and target mappings remain unchanged, including
the nineteen accepted Beach bangle-edge exceptions.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The fresh corpus is `extracted/juniper-beach-finish-study`. All 572 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-beach-swim-trial` bundle. Pins remain strict.
An independent archive inventory exactly matches the profile's eighteen Beach
paths; see `tmp/juniper-beach-finish-author-folder-check.log`.

The new paths use `assets/animations/NPCs/Juniper/Sprites/Beach/` and prefix
`spr_npc_juniper_specialanimation_beach_`. Each retains four 80×80 frames at
duration `0.15`, Default atlas and numeric origin `40.0,54.0`. Raw sidecars in
`tmp/juniper-beach-finish-author-metadata.json` match independent archive reads.
The native swimming-idle pack is linear North/South/East with West mirroring;
the swimming spell pack is linear South. Both are Beach-specific. The shared
slice probes those packs through the native animator with simulated engine
services. Natural spell-event dispatch remains untested.

Every actual frame was inspected in Vanilla and all four target palettes.
Idle North exposes only two ear pixels per frame. Idle East/South expose the
face and ears, while casting additionally exposes both hands and moving
forehead edges. Pink water `EB96CB`, foam `FFD5FB`, detached effects, hair,
eye details and the gold hair tie remain original. The tie's `DF8D4B` and
`FFD565` colors are distinct from skin. No new palette roles or connected
material exceptions are needed. All source alpha and all non-skin pixels
are preserved.

In North frames 1–2, the ear is `[35,51]` and `[35,52]`; in frames 3–4 it moves
down one pixel. Casting frame 1 covers separate `763F21` forehead components
at `[38,45]` / `[39,45]` and `[37,46]`, both raised fingers at `[32,52]` and
`[47,52]`, and the hand shadows beside the foam. Frame 2 exposes a different
forehead corner at `[36,47]` / `[36,48]`. Purple eyelid cosmetics, moving hair
across the cheeks and pink foam touching the hands remain unchanged. Frame
numbers here are one-based and coordinates are frame-local.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Swim idle East | 29, 29, 24, 24 |
| Swim idle North | 2, 2, 2, 2 |
| Swim idle South | 32, 32, 26, 26 |
| Swim spell cast South | 79, 78, 73, 68 |

The masks add 147 seeds and recolor 528 pixels per target. Evidence:
`tmp/juniper-beach-finish-author-refined-components.json` and four refined
four-palette art sheets. Profile SHA-256:
`90b0b41ea703923e13bf6f9a7d457af144d0a66bfae5763824307742836a7b30`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-beach-finish-preview/summary.png)
shows North idle frame 3, East idle frame 3 and spell casting frame 2. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-beach-finish-preview/blue-review/index.html)
includes all sixteen source frames and four West mirrors across five detail
pages, with enlarged face and hand views. The full crop `[24,36,34,28]`
contains every visible source pixel, including detached effects.

Exact checks passed for 3,642,760 preview pixels, forty full bindings,
fifteen summary bindings, source metadata, frame counts and native mirrors.
Chromium checked both indexes and all five detail pages: all local links and
images loaded without horizontal overflow. The complete preview directory is
327,839 bytes; the largest PNG is 49,280 bytes. Evidence:
`tmp/juniper-beach-finish-author-preview-checks.log` and
`tmp/juniper-beach-finish-author-preview-browser-check.json`.

The candidate material test passed, followed by targeted Clippy and the final
focused test in `tests/juniper_beach_finish.rs` (24.62 seconds). It checks
every new RGBA pixel in all four targets, per-frame coverage, unchanged
metadata, 74 literal material landmarks and all 2,288 prior variant
PNG/metadata files. Three omission controls independently remove forehead,
hand and jaw seeds. A spill control deliberately maps pink foam without
region restrictions and changes both adjacent foam and a detached splash.
These mutations demonstrate effective detection and stay in test-local
recipes. Evidence: `tmp/juniper-beach-finish-author-candidate-test.log` and
`tmp/juniper-beach-finish-author-final-checks.log`.

All 1,160 final variants passed exact-palette validation, including the
stylized Debug Blue recipe. All 2,860 earlier original/variant PNG and
metadata files are byte-identical; all 2,320 final variant files match the
inspected candidates. Evidence: `tmp/juniper-beach-finish-author-preservation.log`,
`tmp/juniper-beach-finish-author-stylized-validation.json` and
`tmp/juniper-beach-finish-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes
and isolated installation. Live gameplay and transitions were not exercised
here. All game-derived art remains ignored; nothing was staged or committed
by this author.
