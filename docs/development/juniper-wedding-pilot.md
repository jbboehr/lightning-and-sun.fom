# Juniper Wedding idle and walk

This pilot adds six Wedding strips: idle and walk North, South and East.
There are fifteen source frames and five native West mirrors, giving twenty
review cases. Juniper now has 296 sources and 1,184 variants. All 290 earlier
region objects, pins, seventeen source colors, nine groups and target mappings
remain unchanged, including the nineteen accepted Beach bangle-edge exceptions.

## Sources and materials

The updated read-only archive is `tmp/fields-of-mistria/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
The fresh corpus is `extracted/juniper-wedding-pilot-study`. Despite the archive
update, all 580 earlier original PNG/metadata files match the retained
`characters-reina-juniper-beach-finish-trial` bundle, and all 290 PNG pins still
match. No old hashes or masks were refreshed.

The new paths use `assets/animations/NPCs/Juniper/Sprites/Wedding/` and prefix
`spr_npc_juniper_wedding_`. Idle retains its one-frame defaults. Walk retains
four frames at duration `0.15`. All use 80×80 frames, Default atlas and origin
`Middle,54.0`. Raw sidecars in
`tmp/juniper-wedding-pilot-author-metadata.json` match independent archive reads.
The shared slice checks native linear idle/walk packs and East-to-West mirroring.

Every actual frame was inspected in Vanilla and all four target palettes.
The masks cover face, ears, the dress keyhole, exposed arms, hands, legs and
toes. North views expose only small hand and moving-foot fragments around the
hair and hem. Hair, pink fabric, eye details and bright gold stay original.
Gold hairpin borders reuse `BC8B43`; dark arm-band edges reuse `763F21`, and
anklet borders reuse `BC8B43`. Separable instances are excluded individually.
The distinct gold shades `FFF45D`, `DF8D4B`, `B65932`, `F7C155` and `C97531`
remain unchanged without adding palette roles.

Six acknowledged anklet-edge occurrences follow skin: `[38,52]` and `[39,52]`
in idle East and walk East frames 1 and 3. Their `BC8B43` component also contains
bare-foot shadow `[38,53]`, so the existing component masks cannot separate
them. The established full-skin compromise keeps the foot covered; the opposite
anklet border and every separable gold border stay original. The focused test
asserts these six exceptions explicitly, and every gallery page notes them.
Frame numbers here are one-based and coordinates are frame-local.

Other literal boundaries include South forehead `[38,33]`, keyhole `[39,42]`,
hand `[32,46]` and toes `[37,53]`, which all change. South hairpin border
`[35,30]`, arm-band edge `[34,44]` and anklet border `[37,52]` stay original.
In North walk frame 2, the tiny hand at `[33,48]` and foot at `[41,54]` /
`[42,54]` change while the surrounding hair and dress remain unchanged.

| Strip | Recolored pixels per frame, per target |
| --- | --- |
| Idle North | 2 |
| Idle South | 59 |
| Idle East | 57 |
| Walk North | 2, 3, 2, 3 |
| Walk South | 59, 62, 59, 63 |
| Walk East | 57, 66, 57, 56 |

The masks add 302 seeds, recolor 607 pixels per target and protect 76 material
pixels sharing skin shades. Source alpha is preserved. Evidence:
`tmp/juniper-wedding-pilot-author-refined-components.json` and six refined
four-palette art sheets. Profile SHA-256:
`8b60540dd539dc451e821d4d535d4b31eb44d08476c3a342ee9f6744022a69c0`.

## Offline review and verification

The [five-choice summary](../../generated/juniper-wedding-pilot-preview/summary.png)
shows idle South, walk East frame 1 and walk North frame 4. The
[complete Vanilla/Debug Blue gallery](../../generated/juniper-wedding-pilot-preview/blue-review/index.html)
includes all fifteen source frames and five West mirrors across eight detail
pages. The full crop `[25,20,32,39]` includes every visible pixel. Enlarged
`[28,30,24,26]` views reach the feet to make the anklet compromise visible.

Exact checks passed for 3,862,560 preview pixels, forty full bindings,
fifteen summary bindings, source metadata, frame counts and native mirrors.
Chromium checked both indexes and all eight detail pages: every local link
and image loaded without horizontal overflow. The preview directory is
421,032 bytes; the largest PNG is 60,810 bytes. Evidence:
`tmp/juniper-wedding-pilot-author-preview-checks.log` and
`tmp/juniper-wedding-pilot-author-preview-browser-check.json`.

The candidate material test passed, followed by targeted Clippy and the final
focused test in `tests/juniper_wedding_pilot.rs` (24.93 seconds). It checks
every new RGBA pixel in all four targets, per-frame coverage, unchanged
metadata, 63 literal landmarks, all 76 protected same-shade material pixels,
the six accepted anklet exceptions and all 2,320 prior variant PNG/metadata
files. Four omission controls independently remove forehead, keyhole, hand
and toe seeds. An unrestricted-mapping control changes protected hairpin,
arm-band and anklet pixels, demonstrating effective spill detection. These
mutations remain in test-local recipes. Evidence:
`tmp/juniper-wedding-pilot-author-candidate-test.log` and
`tmp/juniper-wedding-pilot-author-final-checks.log`.

All 1,184 final variants passed exact-palette validation, including the stylized
Debug Blue recipe. All 2,900 prior original/variant PNG and metadata files are
byte-identical; all 2,368 final variant files match the inspected candidates.
Evidence: `tmp/juniper-wedding-pilot-author-preservation.log`,
`tmp/juniper-wedding-pilot-author-stylized-validation.json` and
`tmp/juniper-wedding-pilot-author-final-diff-check.log`.

The shared slice handles broader checks, combined packaging, native probes
and isolated installation. Live gameplay and transitions were not exercised
here. All game-derived art remains ignored; nothing was staged or committed
by this author.
