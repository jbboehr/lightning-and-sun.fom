# Juniper's first overworld batch

This pilot adds six Spring idle/walk strips: fifteen source frames and five
native West mirrors, making twenty review cases. Page Down selects Vanilla,
Debug Blue, Hayden, Ryis or Seridia for Juniper's portraits and these world
sprites. Other actions and outfits remain original.

The new `juniper-world-trial.json` profile and set retain all 132 portrait
regions and add six world regions, for 138 sources and 552 recolored variants.
The portrait-only definitions remain untouched. Existing source colors, color
groups, source hashes and target mapping roles are preserved; one new world
shade is appended. Shared Rust and GML runtime behavior is unchanged.

## Source and mask decisions

Sources are under `assets/animations/NPCs/Juniper/Sprites/Spring/`, named
`spr_npc_juniper_spring_{idle,walk}_{north,south,east}.png`. All retain 80×80
frames, Default atlas and Middle/54 origin. Idle uses the single-frame defaults;
walking has four frames at duration `0.15`. Raw exported metadata in
`tmp/juniper-world-author-metadata.json` matches the independent root inventory.
The read-only source is `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
The full local corpus is `extracted/juniper-world-study`.

World skin uses existing portrait colors `EFD89A`, `E3BF7F` and `763F21`, plus
new `BC8B43`. The new shade maps to the existing third target shade and has its
own color group. The twelve existing source colors, four original groups and
all 132 prior region objects stay unchanged. Keeping the new group separate
prevents a hand mask from crossing into an adjacent same-shade gold bracer.

Every source frame was inspected in Vanilla and all four targets. Face, ears,
neck, exposed chest and midriff, upper arms, hands and visible thighs change.
Circlet borders, bracers and trailing skirt trim reuse the skin ramp: their
reviewed components remain original. This includes the `E3BF7F` shadows of
turned bracers and `763F21` at the trailing East skirt point. Purple hair,
eyes, blue gemstones, bodice, skirt fabric, bright gold and boots are preserved.
The original Spring neutral portrait provided additional outfit context.

| Strip | Skin pixels per frame, per target |
| --- | --- |
| Idle East | 41 |
| Idle North | 11 |
| Idle South | 50 |
| Walk East | 41, 46, 41, 37 |
| Walk North | 11, 9, 11, 8 |
| Walk South | 50, 48, 50, 42 |

The six masks use 254 seeds and select 496 skin pixels per target; 95 matching
skin-ramp pixels on clothing and accessories remain excluded. Evidence:
`tmp/juniper-world-author-refined-components.json`,
`tmp/juniper-world-author-exclusions.json` and the six
`tmp/juniper-world-author-refined-art-{cycle}_{direction}.png` sheets.
Profile SHA-256:
`3266ca08b3ebfdf2cd7131ab3e01cf7de9dfcd5036b60f10383f19d91fb38861`.

## Offline review

The [five-choice summary](../../generated/juniper-world-preview/summary.png)
shows idle South, walk East and walk North, in about 47 KB. The
[complete Vanilla/Blue review](../../generated/juniper-world-preview/blue-review/index.html)
covers every source frame and native West mirror across eight detail pages,
with at most four cases per page. Each case also includes enlarged face detail.

Crop `[25,20,32,39]` contains every visible source pixel, including swinging
hair, hands, boots and West mirrors. Exact checks cover 4,231,200 preview
pixels, all forty full-frame and fifteen summary bindings, metadata and native
West reversal. Chromium checked all ten pages, including both indexes; every
image and local link loaded, with no horizontal overflow. Evidence:
`tmp/juniper-world-author-preview-check.log` and
`tmp/juniper-world-author-preview-browser-check.json`.

## Verification

Candidate structural and material tests passed before promotion. Final
`tests/juniper_world.rs` passed targeted Clippy, the structural test and opt-in
local test.
The structural test protects all portrait regions, groups, source colors and
target roles. The local test checks every new pixel in all four targets,
per-frame counts, metadata, 35 literal material landmarks and all prior
portrait variant bytes. Practical controls remove a hand component and merge
the groups: the former omits hand skin, and the latter recolors a protected
bracer shadow. Log: `tmp/juniper-world-author-final-test-checks.log`.

The final standalone bundle preserves all 1,320 prior original and variant
PNG/metadata files byte-for-byte; all 552 variants pass exact-palette
validation. Evidence: `tmp/juniper-world-author-preservation.log`. Source hashes
remain strict; no old pin was refreshed or bypassed.

Shared collection packaging, native probes and installation are handled by
the combined pilot slice. Static previews do not establish natural schedules,
animation timing, outfit changes or live gameplay. The user approved this artwork for commit on 2026-09-25. Extracted and generated game artwork remains ignored.
