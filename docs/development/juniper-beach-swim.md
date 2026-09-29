# Juniper Beach bathing and swimming

This slice adds only `bath_swim_east` and `bath_swim_south`: two strips with
eight source frames and four native West mirrors, giving twelve review cases.
Juniper now has 286 sources and 1,144 variants. All 284 prior region objects,
source pins, seventeen source colors, nine groups and target mappings remain
unchanged, including the nineteen accepted Beach bangle-edge exceptions.
Special swimming and spell sprites remain outside this slice.

## Sources and materials

The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The fresh corpus is `extracted/juniper-beach-swim-study`. All 568 earlier
original PNG/metadata files match the retained
`characters-reina-juniper-march-beach-actions-trial` bundle. Pins remain strict.

Both paths use `assets/animations/NPCs/Juniper/Sprites/Beach/` and prefix
`spr_npc_juniper_beach_`. They retain four 80×80 frames at duration `0.15`,
Default atlas and origin `Middle,54.0`. Raw sidecars in
`tmp/juniper-beach-swim-author-metadata.json` match independent archive reads.
The shared slice checks native directions, timing and East-to-West mirroring.

Every actual frame was inspected in Vanilla and all four target palettes.
Only the exposed face and ears change; the waterline covers the body and wrists.
Hair, eye details and the gold hair tie remain original. The tie's `DF8D4B` and
`FFD565` shades are distinct from the skin palette, as are `328BC9` water and
`9DEBFC` foam. No new palette roles or connected-material exceptions are needed.
All source alpha values and all non-skin pixels are preserved.

In East frame 1, `[38,54]` is a `763F21` jaw shadow and `[40,54]` is `E3BF7F`
lower-face skin; both change. The touching foam at `[40,55]` stays original.
South frame 1 separately covers the forehead shadow `[38,48]`, both ears
`[35,51]` / `[44,51]` and both jaw corners `[37,54]` / `[42,54]`. The lower pose
in frame 3 retains its face down to row 54 while preserving the foam immediately
below it. Frame numbers in this document are one-based and coordinates are
frame-local.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Bath/swim East | 29, 29, 24, 24 |
| Bath/swim South | 32, 32, 26, 26 |

The masks add 64 seeds and recolor 222 pixels per target. Evidence:
`tmp/juniper-beach-swim-author-refined-components.json` and the two refined
four-palette art sheets. Profile SHA-256:
`dbf039b6efc5b213e8746277c32deb33a40b78f09c95490bcdeda2ebc8bb6f37`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-beach-swim-preview/summary.png)
shows South frame 1, East frame 3 and South frame 4. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-beach-swim-preview/blue-review/index.html)
includes all eight source frames and four West mirrors across three detail
pages, with enlarged face views. The full crop `[24,36,34,28]` includes all
visible pixels, including detached splashes below the standing-sprite crop.

Exact checks passed for 2,328,456 preview pixels, twenty-four full bindings,
fifteen summary bindings, source metadata, frame counts and native mirrors.
Chromium checked both indexes and all three detail pages: all local links and
images loaded without horizontal overflow. The complete preview directory is
216,913 bytes; the largest PNG is 47,462 bytes. Evidence:
`tmp/juniper-beach-swim-author-preview-checks.log` and
`tmp/juniper-beach-swim-author-preview-browser-check.json`.

The candidate material test passed, followed by targeted Clippy and the final
focused test in `tests/juniper_beach_swim.rs` (30.55 seconds). It checks every
new RGBA pixel in all four targets, per-frame counts, unchanged metadata,
38 literal material landmarks and all 2,272 prior variant PNG/metadata files.
Four effective controls omit a forehead, ear or jaw component, then deliberately
add a foam palette entry without region restrictions; the latter changes both
adjacent foam and a detached splash, proving that the protection assertions
catch water spill. These mutations stay in test-local recipes. Evidence:
`tmp/juniper-beach-swim-author-candidate-test.log` and
`tmp/juniper-beach-swim-author-final-checks.log`.

All 1,144 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,840 earlier original/variant PNG and metadata files
are byte-identical; all 2,288 final variant files match the inspected candidates.
Evidence: `tmp/juniper-beach-swim-author-preservation.log`,
`tmp/juniper-beach-swim-author-stylized-validation.json` and
`tmp/juniper-beach-swim-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes and
isolated installation. Live gameplay and transitions were not exercised here.
All game-derived art remains ignored; no game files were staged or committed.
