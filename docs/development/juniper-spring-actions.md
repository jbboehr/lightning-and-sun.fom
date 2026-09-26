# Juniper's Spring regular actions

This slice adds eleven Spring strips: blink East/South, and sit, eat and drink
North/South/East. Their 31 source frames and twelve native West mirrors make
43 review cases. The profile now has 149 sources and 596 variants. All 138
previous region objects and source hashes remain unchanged, including the
accepted idle/walk masks. Portrait-only definitions remain untouched.

## Source and palette decisions

The read-only archive is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The complete local corpus is `extracted/juniper-spring-actions-study`. All 276
earlier original PNG/metadata files match the retained first-world combined
bundle; no old pin was refreshed or bypassed.

New sources under `assets/animations/NPCs/Juniper/Sprites/Spring/` use the prefix
`spr_npc_juniper_spring_`. All retain 80×80 frames, Default atlas and Middle/54
origin. Blink uses `[0.075, 0.125, 0.075]`; sitting uses single-frame defaults;
drink and eat North use three frames with duration `1.0`; eat East/South use
five frames with `[0.125, 0.15, 0.175, 0.125, 0.6]`. Raw sidecars in
`tmp/juniper-spring-actions-author-metadata.json` match the root inventory.

The existing thirteen source colors, five color groups and target mapping roles
are preserved. One shade is appended: `E8B171`, used by 23 exposed hand,
forearm and chest pixels in drink East/South. It has a separate singleton
group and maps to the existing second target shade, alongside `E3BF7F`.
Its addition leaves all earlier portrait and world outputs byte-identical.

Every actual frame was inspected in Vanilla and all four targets. Face, ears,
eyelids, neck, chest, midriff and exposed arms/hands change. Circlet borders,
bracers and skirt trim reuse skin shades; reviewed components protect those
materials, subject to the three small exceptions below. Hair, eyes, gemstones,
bodice, skirt fabric, boots, mouth interiors and black outlines remain original.

| Strip | Skin/component pixels per frame, per target |
| --- | --- |
| Blink East | 41, 45, 41 |
| Blink South | 50, 54, 50 |
| Drink East | 30, 32, 30 |
| Drink North | 7, 4, 7 |
| Drink South | 35, 39, 35 |
| Eat East | 27, 31, 29, 27, 32 |
| Eat North | 7, 4, 7 |
| Eat South | 36, 38, 31, 40, 32 |
| Sit East | 28 |
| Sit North | 10 |
| Sit South | 32 |

The new masks use 435 component seeds and recolor 911 pixels per target;
191 matching-color clothing/accessory pixels remain excluded. Evidence:
`tmp/juniper-spring-actions-author-refined-components.json`,
`tmp/juniper-spring-actions-author-exclusions.json` and the eleven
`tmp/juniper-spring-actions-author-refined-art-{cycle}_{direction}.png` sheets.
Profile SHA-256:
`4f8619d030320c8ea23322901c6925a84d85861e088df48df51ca0fc59b5caec`.

## Known art exceptions

Three material-edge pixels connect to exposed skin within the existing color
groups. The current mask format selects complete components and cannot clip
these edges independently. This slice keeps the skin component, so these
three edge pixels also change. Frame numbers below are one-based; coordinates
are local to the 80×80 source frame.

| Source | Frame | Pixel | Material edge |
| --- | --- | --- | --- |
| Eat South | 2 | `[40,46]`, `BC8B43` | Hem touching midriff |
| Eat South | 3 | `[41,45]`, `BC8B43` | Clasp touching midriff |
| Eat East | 5 | `[39,44]`, `E3BF7F` | Bracer touching forearm |

The local test names these exceptions explicitly. No mask schema or runtime
behavior was changed to accommodate them.

## Offline review and verification

The [five-choice summary](../../generated/juniper-spring-actions-preview/summary.png)
shows eat South frame 2, eat East frame 5 and drink East frame 2. It includes
two known edge exceptions and an example of the new arm shade. The
[complete Vanilla/Blue gallery](../../generated/juniper-spring-actions-preview/blue-review/index.html)
covers all 43 cases across fifteen detail pages, with at most five cases per
page. Each case also has an enlarged face detail.

Crop `[25,20,32,39]` contains every visible pixel, including extended hands,
hair, boots and West mirrors. Exact checks verified 8,558,880 preview pixels,
86 full-frame and fifteen summary bindings, source metadata and West reversal.
Chromium checked all seventeen pages, including both indexes: every image
and local link loaded, with no horizontal overflow. Evidence:
`tmp/juniper-spring-actions-author-preview-check.log` and
`tmp/juniper-spring-actions-author-preview-browser-check.json`.

Candidate tests passed before promotion. Final targeted Clippy and both tests
in `tests/juniper_spring_actions.rs` passed. They check append-only palette
roles, all new pixels in four targets, per-frame counts, metadata, 37 literal
landmarks, the 23 alias pixels and all 1,104 previous variant PNG/metadata
files. Practical controls demonstrate omitted arm-midtone pixels and merged
groups spilling into a protected bracer shadow. Evidence:
`tmp/juniper-spring-actions-author-final-test-checks.log`.

All 596 final variants pass exact-palette validation, and all 1,380 previous
original/variant PNG and metadata files remain byte-identical. Evidence:
`tmp/juniper-spring-actions-author-preservation.log`. The shared slice handles
combined packaging, native probes and installation. Static source previews do
not establish attached prop behavior, animation timing, natural schedules,
outfit changes or live gameplay. The user approved this artwork for commit on 2026-09-25. Game-derived
images remain ignored.
