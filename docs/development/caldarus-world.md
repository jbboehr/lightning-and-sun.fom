# Caldarus world artwork

The eighth finish batch adds all 179 adult Caldarus world strips. The candidate
retains every one of the 98 prior portrait regions, including the three deliberate
dragon/statue no-change regions. It therefore contains 277 hash-bound regions.
Displayed world artwork is accepted after the consolidated review; inspection
depth is unspecified and live-game acceptance remains pending. The parent
batch owns collection integration, full checks, install/remove verification and the
single combined user review.

The [eighth-batch record](eighth-finish-batch-2026-11-06.md) owns the user's
2026-11-06 assessment and combined verification. The accepted regression snapshot
is `local/fixtures/finish-caldarus/`; retaining portraits does not broaden the
displayed world-art approval.

## Inventory and source

Source is the preserved `local/game/assets.zip`, SHA-256
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
Every one of the 554 selected source PNG/metadata members matches that archive
byte for byte. No archive copy or installation was made by this author.

| Outfit | Strips | Native frames | West mirrors | Selected skin pixels per target |
| --- | ---: | ---: | ---: | ---: |
| Autumn | 37 | 125 | 40 | 6,399 |
| Beach | 16 | 56 | 24 | 3,578 |
| Spring | 37 | 125 | 40 | 6,565 |
| Summer | 37 | 125 | 40 | 6,737 |
| Wedding | 15 | 49 | 20 | 2,112 |
| Winter | 37 | 125 | 40 | 5,964 |
| Total | 179 | 605 | 204 | 31,355 |

The 605 native occurrences contain 415 distinct exact frames. The 204 horizontal
West mirror occurrences contain 139 distinct exact frames. There are 809 complete
review cases. Every native frame exposes skin; no new world region has an empty
mask. The complete native inventory includes seasonal action, blink, eating,
drinking, idle, kissing, sitting, sleeping and walking, plus eyes-closed, harvest,
lute start/loop/end, standing/seated reading start/loop/end and spell start/loop/end.
Beach includes both bathing/swimming directions; Wedding has its native smaller
inventory. No West source strip is invented.

All new native metadata uses the `Default` atlas, 80×80 frames, horizontal
`Middle` and vertical `54.0` offsets. Original frame counts and scalar/array timing
values remain unchanged. The stable source corpus is
`extracted/test-corpus/caldarus`, pointing to
`extracted/caldarus-world-finish-study`.

## Materials and special forms

The existing nine source roles, singleton groups, preset IDs/labels and their
mappings remain exact. The world art uses the original light `#C1AFA5`, medium
`#A69084`, shadow `#8E746D` and deep `#624A48` roles. One extra source shade,
`#AF8D81`, occurs on the Beach East walk frame 2 calf at `[40,51]`. Its new singleton
group maps to the existing medium target in every preset; it adds no new tone or
choice. The candidate has 18,579 new world seeds.

Selections cover forehead/hairline shading, exposed face and ears, skin beside
mouths, neck, Beach chest and arms, hands/fingers, legs and feet. White nails and
the white/pale-pink body markings retain their source colors. In particular,
`#F5F5F5` occurs in both tail/horn ivory and human body markings; a whole connected
multicolor cluster must not be treated as a protected material just because it
contains white. Source inspection separated the marked Beach forearm from the
scaled tail, and all base skin around the markings changes. The five `#D7BBBB`
Beach body-marking transition pixels stay original, as do `#E7D5D5` transitions.

Horns, tail scales and spines deliberately retain their original ivory/tan colors,
even where they share human skin shades. Across this world corpus, 17,382 pixels
using the four original skin shades belong to those protected materials. Teal
hair, gray eye details, eyes, jewelry, clothing, scrolls, lute, music notes,
food/drink and genuine red/dark mouth interiors also remain original. White tail
spines and horn material remain distinct from the recolored face and hands.

The form audit covers all six human world outfits, with Beach and Wedding
explicitly inspected. There are no additional adult dragon/statue world strips
in this 179-strip native inventory. The retained three portrait dragon/statue
regions remain deliberate no-ops with their original empty masks and exact PNGs;
this does not introduce a separate dragon-scale recoloring design. Children are
outside the approved adult NPC inventory.

Caldarus's five existing choices are Vanilla, Debug Blue, Adeline, Hayden and
Ryis. His native skin is the Seridia ramp, so Vanilla already supplies that choice.
All 784 retained portrait PNG/metadata files (98 strips × four targets × two
members) are byte-identical to a fresh prior-profile release rebuild, including
all unchanged dragon/statue copies.

## Independent checks

[The world tests](../../tests/caldarus_world.rs) contain a literal native inventory,
per-frame selected totals and source-inspected horn/tail exclusions. They compute
expected replacements independently of stored seeds, compare every decoded pixel
and alpha in all four targets, verify complete native metadata and compare every
retained portrait PNG/metadata member. Separate literal landmarks cover each skin
role, ears, hand contours, horns, tail, eyes, hair, jewelry, white markings/nails,
the rare calf shade and true mouth interiors.

The focused release checks passed two normal inventory/package cases, the new
full-material/retained-output case and the prior portrait material case. Scoped
Clippy with warnings denied and Rust formatting passed. Four exact recipe
validations passed for Blue, Adeline, Hayden and Ryis. Reference-mask analysis
reused all 605 occurrences in 415 exact groups with zero conflicts and no
unreviewed groups.

Two deliberate local negative controls failed at the intended independent
material landmarks: omitting the Autumn South idle hand seed `[45,43]` left skin
uncolored; adding horn seed `[36,30]` changed protected ivory/tan material. These
candidates and failure logs are retained locally. An initial test landmark was
transcribed as skin at `[27,42]`; the source proves black there. Correcting the
literal source assertion did not change the candidate artwork.

Evidence under `local/authoring/caldarus-world/` includes `tests-final.log`,
`clippy.log`, the four `validation-*.json` results,
`independent-verification.json`, `retained-output-verification.json`,
`material-evidence.json` and the negative-control logs. The combined batch owns
fresh repository-wide checks and installation/removal; this author did not test
live gameplay or claim visual acceptance.

## Offline review

The complete author gallery is
[`generated/caldarus-world-finish-review/index.html`](../../generated/caldarus-world-finish-review/index.html).
Vanilla is left and Debug Blue right; each case includes a full 80×80 sprite and
an enlarged detail. Every native frame and West mirror is present, split by six
outfits and at most four frames per detail page. The compact overview is explicitly
a sample: columns are Vanilla, Blue, Adeline, Hayden and Ryis.

Every placed pixel in the final gallery was checked against the actual current
source or bundle: 216,954,280 exact bound pixels, covering 1,618 full comparisons,
1,618 enlarged details and 30 five-choice sample cells. Full sprites preserve the
entire native frame; the enlarged crop `[24,19,35,44]` also covers the union of
visible world-art bounds. All 415 distinct native and 139 distinct West source/Blue
pairs were visually inspected in full and enlarged views, with exact duplicates
bound by the gallery and reference analysis. All six five-choice samples were
also inspected.

CPU Chromium passed 310 desktop/mobile page loads: every local link and image
loaded, with no horizontal overflow. Four final root/Beach browser screenshots
were visually inspected. This verification and frozen final input/output hashes are
recorded in `verification.json` and `final-input-and-output-hashes.json` under the
same author directory. No local GPU rendering or model inference was used.

## Local retention

The complete source export, candidate bundle, mask-reference review, user-facing
author gallery, scripts, material evidence and inspection contacts are retained.
The source export is about 1.5 MB, bundle 44.4 MB, author gallery 15.6 MB and mask
review 11.6 MB of file bytes; filesystem allocation is larger because many source
members are small. Source grids and final full/enlarged inspection contacts were
moved into `local/authoring/caldarus-world/inspection-artifacts/` (about 34.4 MB)
so they remain evidence without leaving loose PNGs in `tmp/`. No prior accepted
fixture, source snapshot or shared artifact was edited or deleted.

After verification, 34.1 MB of disposable compiled helper executables were
removed; their Rust sources and all source/visual evidence remain retained.
