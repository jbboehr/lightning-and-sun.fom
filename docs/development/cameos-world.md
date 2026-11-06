# Darren, Linnet, Wiscar and Wynne: complete world candidate

2026-11-05. **Authored and locally validated; user visual acceptance pending.**
This approved P2 batch covers all 45 omitted cameo world strips. Four new
profiles/sets retain all eight portraits per character, previous source
roles/groups and preset mappings. Shared registry, collection selection, counts
and combined verification belong to the integrator. No runtime changes.

## Source inventory

The source archive is local/game/assets.zip, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
All 154 exported PNG/metadata members match the archive exactly. Every source
PNG is hash-pinned. Native definitions come from assets/fiddle/cameos/; world
PNGs live directly under each character's Sprites/ folder without seasonal
subfolders. Their names and native definitions identify Spring.

| Character | New world strips | Total regions | Native frames | Unique frames | West mirrors | Selected pixels per target |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Darren | 11 | 19 | 24 | 16 | 9 | 755 |
| Linnet | 12 | 20 | 31 | 19 | 16 | 830 |
| Wiscar | 11 | 19 | 24 | 16 | 9 | 661 |
| Wynne | 11 | 19 | 24 | 16 | 9 | 1,019 |
| Total | 45 | 77 | 103 | 67 | 43 | 3,265 |

Every character has blink East/South (three frames), idle and sit
East/North/South (one), and walk East/North/South (four). Linnet additionally has
seven action East frames. No native eating strips occur. The 36 duplicates
reuse newly authored candidate masks, not accepted world masks. New seed counts
are Darren 385, Linnet 431, Wiscar 302 and Wynne 570.

Wiscar's idle East/North/South and walk North metadata use PortraitsMisc. His
other seven world strips and all other cameo world strips use Default. Packages
preserve those original atlas assignments, frames, scalar/array timing, offsets
and pixels. West cases use the native horizontal mirror.

## Material decisions

**Darren:** existing #A3614E, #83453A, #63342A and #491F1B select exposed face,
ears, back-neck shading and hands. Beard/head hair, eyes, cream cuffs, jacket,
trousers, shoes and shirt buttons stay original. Exactly 18 shirt-button pixels
share #83453A. Walking raises hands to row 44 and moves back-neck shading to row
40, so a coarse body cutoff is insufficient. The authoring helper uses explicit
button exclusions; tests independently check their pose-specific positions.
No new colors or target roles.

**Linnet:** existing #C7875C and #4A1B16 plus new #A85E44 (medium), #7F3E30
(shadow) and #B97551 (existing medium role) cover face, ears, neck and the brown
transition beside the pink scar. Source pink scar colors #FFA793 and #EF977D
remain original. Gold circlet/trim, white/lavender hair, eyes, armor, gloves,
sword and equipment stay original. Hands/feet are covered. Idle North, sit
North and walk North contain no visible skin: three intentional empty masks
cover six frames. Their complete pixels remain unchanged in every target;
independent rear source views show hair, armor and gloves. Other strips have skin.

The user asked whether Linnet's apparently unrecolored patch is her scar, then
said that treatment was expected and requested a commit if confirmed. Comparing
the original portrait and world sprite confirms the peach/pink scar; fresh
Linnet portrait/world material tests pass, including preserved `#FFA793` and
`#EF977D` landmarks and recolored adjacent skin. No artwork changed. Record this
assessment for the scar treatment without inferring broader batch acceptance.

**Wiscar:** world skin uses existing light #E3A17B, deep #9F5544 and #6D3328,
plus new medium #CC795D. The existing two-thirds-deep rule for #6D3328 is
retained. Face, ears and hands change, including the small East fingertip and
seated hands. Pink hair/beard, eyes, circlet, white robe and gold trim stay
original. All world #D98364 pixels are garment/trim, despite their portrait
skin role. Another 67 #6D3328 robe/trim pixels are protected while matching hand
shadows change. Authoring identifies positive hand components; the independent
test instead lists protected garment coordinates and checks all remaining skin.

**Wynne:** existing #A3614E, #83453A and #491F1B plus new shadow #68332C cover
face, ears, neck, hands and fine forehead/jaw/hand contours. Dark hair, eyes,
gold earrings/nails, blue clothing and footwear stay original. The new brown
shade also occurs on hands and forehead; it is anatomical shading rather than
a cosmetic mouth color. Existing portrait cosmetics and custom target formulas
remain exact.

Counts describe selected source material. Some Darren/Wynne Ryis mappings are
identities, so selected pixels need not visibly change. Prior role semantics
remain unchanged.

## Offline review

- [Group review index](../../generated/cameos-world-finish-review/index.html).
- Complete character reviews:
  [Darren](../../generated/darren-world-finish-review/index.html),
  [Linnet](../../generated/linnet-world-finish-review/index.html),
  [Wiscar](../../generated/wiscar-world-finish-review/index.html),
  [Wynne](../../generated/wynne-world-finish-review/index.html).

Each correctly named five-choice SAMPLE summary shows Vanilla, Debug Blue,
Hayden, Ryis and Seridia. Complete pages show Vanilla left and Debug Blue right,
full visible sprites and enlarged face/hand details. There are 146 cases across
64 case pages and eight indexes. Full crops contain every opaque pixel,
including Linnet's sword. The author inspected all native source/candidate
cases and all West source/Blue mirrors, with coordinate views for shared
materials, face/mouth boundaries and hand details. This is author inspection.

Sheets use the actual final bundles. Exact checks verify 20,454,528 placed
pixels, 292 complete Vanilla/Blue bindings, 292 detail views and 20 sample
cells, with source-metadata reconciliation and full visible bounds. CPU Chromium
passed all 72 detailed desktop pages, eight mobile indexes and the group index
at both widths: 82 loads, every image/link present, no horizontal overflow.
The four character runs took 106.11s combined under concurrent CPU load.
Summary titles were visually inspected; scoped text checks found no copied
other-character names.

A later parent spot-check prompted a direct native Linnet/Wiscar label audit.
Before any edits, both PNGs already contained the complete SAMPLE titles and
all five column names: 88,000 header pixels per image matched literal expected
glyphs exactly. Deterministic regeneration left both entire galleries
byte-identical. A fresh 11,173,440-pixel binding check and 42 affected CPU browser
page loads passed. Recipe, mask and material-test hashes remain unchanged;
no image correction was needed. Fresh summary SHA pins and detailed evidence
are in `local/authoring/cameos-world/summary-label-audit.json` and review
`check-results.json` files.

## Focused verification

Fresh checks passed:

~~~sh
nix-shell --pure --run 'cargo fmt --check && cargo clippy --release --locked --test cameos_world -- -D warnings && cargo test --release --locked --test cameos_world --test darren_portraits --test linnet_portraits --test wiscar_portraits --test wynne_portraits -- --include-ignored'
~~~

Two regular tests check complete root-level membership, retained portraits/roles
and native package metadata/pixels. Four source-backed tests check every world
RGBA pixel in all targets with independent material rules, fixed frame counts
and 38 literal skin/protected positions. Metadata bytes are preserved. All 256
retained portrait PNG/metadata outputs match old-profile rebuilds byte-for-byte.
The four existing portrait material, mouth, hand, cosmetics and costume tests
also passed unchanged.

All 16 character/preset combinations passed CLI exact-recipe validation over
their complete corpora. Component galleries reconstruct all 103 frame
occurrences with zero conflicts/unresolved groups. Eight deliberate negatives
failed at the intended material assertions:

| Character | Omission control | Material spill control |
| --- | --- | --- |
| Darren | Hand (32,46) | Same-color shirt button (39,42) |
| Linnet | Brown face transition (40,37) | Gold circlet (39,31) |
| Wiscar | East fingertip (45,46) | Same-color robe shadow (36,46) |
| Wynne | East hand shadow (34,47) | Dark hair (39,30) |

The final unmodified tests passed afterward. FOM_CAMEO_WORLD_PRESETS overrides
the set for one explicitly selected source-backed character test.

Current profile/set SHA-256 pins:

| Character | Profile | Set |
| --- | --- | --- |
| Darren | `746a9c74bc6ec09bb90549e9a41fcc11ae99e9950bcceb85d5f06b259efc6bfd` | `e88adcbaa7c15c28559ee37e46262217431c93b0525e710c99294f580c52f6c8` |
| Linnet | `752a007ee247dc2f1ed5aec58369af03ef8aabc8553472c96992e0e31e737f5e` | `8e1f83e1d226140ec54e6021c0ca0b0cbbd041d5a1351164de248529763c5069` |
| Wiscar | `0ea469b509d0eeae1ce7e7f95e9edf3bc58f3b7b9e862a79744e50bd5ea08008` | `30dc3fba828650c8fb41b78a17fa2bb633cd3ad4b8c2b0cbd23d2bca5f2e0a4f` |
| Wynne | `460c54b37a50178affcbaf10f51d1d3fea166c3649caf878009604c9d8fd72f8` | `cfc9bae0bab770c7a8c93871a611aee19fc8404a92ea9d30793caf14e4c0f975` |

Reproduction inputs, native cameo definitions, Rust author/gallery helpers,
source grids, negative recipes/logs and measurements are under
`local/authoring/cameos-world/`. Builds are `generated/<id>-world-finish-build/`;
component galleries are `generated/<id>-world-mask-review/`. Stable corpus links
point to the complete `extracted/<id>-world-finish-study/` exports. Review
`check-results.json` files pin inputs and limits. Historical fixtures are intact.

The source-export start to checked-candidate checkpoint took 16m39s elapsed,
including four material studies, authoring, inspection, focused checks and
browser runs; documentation and combined integration are excluded. The final
six-test group took 0.22s. The separate 16-validations/eight-negatives/source
audit/component-gallery pass took 16.35s. Human review time is unmeasured.
No local GPU, local inference or Pillow was used.

No new artwork is accepted and no accepted world baseline was written.
Combined collection/install/remove checks belong to the parent batch. Native
cameo cutscenes, natural gameplay and rendered/input acceptance remain
unverified; other browsers are unverified. About 25 MiB of scoped source,
build, review and authoring evidence is retained, plus small inspection
sheets/helpers under project tmp/. No archive copies or tracked game imagery.
