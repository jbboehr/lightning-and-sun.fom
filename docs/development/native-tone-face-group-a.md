# Native-tone facial correction — group A

2026-11-06. All 18 owned characters have source-bound portrait corrections.
Technical authoring checks pass; the complete roster review and integrated
package/install checks belong to the parent batch. This is candidate artwork,
not recorded visual acceptance.

The user reported near-universal unscaled blush and sporadic discoloration near
mouths in the all-character expression sheet. This group includes Adeline,
Hayden, Ryis, Reina, Juniper, March, Balor, Valen, Eiland, Olric, Landen, Nora,
Holt, Josephine, Darcy, Dell, Elsie and Errol. Every selected portrait strip,
outfit/expression and native frame was scanned. Source/before mouth crops were
deduplicated by their actual RGBA bytes; all unique patterns were visually
inspected. Actual generated after crops were inspected for the corrected mouths
and the six tone families.

## Material decisions

Blush retains its source shape and receives visible muted rose colors derived
from the improved Celine treatment. Source-specific main/highlight/deep shades
remain distinct, including Adeline's sick-nose treatment, March's broad flustered
patch and Elsie's permanent cheek makeup. The Celine files and outputs were not
part of this group's edits.

Confirmed warm mouth contours are selected in March, Valen, Dell and Nora.
Darcy's pale closed-mouth lip accent uses restrained rose colors while her
open-mouth tongue colors stay exact. Juniper's nose/inner-eye skin accents and
a source skin shade along the outer laughing-mouth edge are selected; her
lavender lipstick and pink tongue stay exact. Adeline has tiny under-eye skin
highlights and a warm cheek-edge pixel corrected separately from her blush.

Nora's existing two cosmetic targets remain exact. Her added orange outer-edge
shades reuse the already reviewed skin-shadow/cosmetic targets. Dell's nose
blush uses an existing genuine skin-role color (`#EC9871`) plus a separate
`#D98863` accent: the existing slot keeps its donor-middle target everywhere,
and the new accent supplies the rose tint. No existing skin-role target changes.
Intentional lipstick, tongue/mouth interiors, teeth, iris/sclera, lashes,
hair/brows/beards, scars, jewelry and clothing remain protected by the exact
pixel-delta proof outside reviewed components.

## Counts and preservation

| Character | Portrait strips audited | Strips with new reviewed components | Source pixels selected | Changed new-tone PNGs |
| --- | ---: | ---: | ---: | ---: |
| [adeline](native-tone-face-corrections/adeline.json) | 126 | 116 | 3250 | 696 |
| [hayden](native-tone-face-corrections/hayden.json) | 133 | 38 | 2052 | 228 |
| [ryis](native-tone-face-corrections/ryis.json) | 109 | 44 | 1408 | 264 |
| [reina](native-tone-face-corrections/reina.json) | 103 | 43 | 1526 | 258 |
| [juniper](native-tone-face-corrections/juniper.json) | 132 | 61 | 2230 | 366 |
| [march](native-tone-face-corrections/march.json) | 181 | 181 | 5163 | 1086 |
| [balor](native-tone-face-corrections/balor.json) | 110 | 42 | 1680 | 252 |
| [valen](native-tone-face-corrections/valen.json) | 92 | 92 | 2473 | 552 |
| [eiland](native-tone-face-corrections/eiland.json) | 78 | 28 | 836 | 168 |
| [olric](native-tone-face-corrections/olric.json) | 36 | 4 | 144 | 24 |
| [landen](native-tone-face-corrections/landen.json) | 32 | 4 | 136 | 24 |
| [nora](native-tone-face-corrections/nora.json) | 32 | 32 | 288 | 192 |
| [holt](native-tone-face-corrections/holt.json) | 32 | 4 | 148 | 24 |
| [josephine](native-tone-face-corrections/josephine.json) | 32 | 4 | 130 | 24 |
| [darcy](native-tone-face-corrections/darcy.json) | 32 | 32 | 416 | 192 |
| [dell](native-tone-face-corrections/dell.json) | 32 | 20 | 192 | 120 |
| [elsie](native-tone-face-corrections/elsie.json) | 36 | 36 | 2376 | 216 |
| [errol](native-tone-face-corrections/errol.json) | 36 | 4 | 142 | 24 |

The group covers **4,017 selected strips**, including
**1,364 portrait strips / 2,728 native
portrait frames**. The full candidates contain **40,170 PNGs**.
All **16,068 older-choice PNGs** and
**15,918 new-choice world PNGs** match the frozen before build
byte for byte. All **40,170 metadata files** remain byte exact.
Every one of the **8,184 new-choice portrait PNGs** was
decoded and compared to the previous PNG with only the explicitly recorded
component targets applied. This proves alpha and unrelated pixels stay exact.
The change affects **4,710 PNGs / 147,099 RGBA pixels**.

Literal source-backed landmarks failed against the frozen before outputs and
passed against the actual after candidates for all six new presets. There are
**162 corrected landmark checks** and
**550 protected landmark checks**.
An additional broad-face scan found no remaining unselected, unchanged source
blush colors for the reviewed material colors in any of the 18 characters.
This check does not classify every warm pixel as skin.

## Reproduction and evidence

Each linked JSON record owns the source PNG hashes, base/new profile paths,
additional unique source colors, full six-preset target arrays, explicit seeded
single-/multi-color components with pixel counts, literal landmarks, protected
material landmarks and the complete portrait audit inventory. Existing profile
source-slot order, inherited groups, region bindings, sizes, hashes and seed
prefixes remain exact. Only the six additions use each new profile; top-level
shared profiles and the four earlier preset objects are unchanged.

Ignored authoring helpers are in `local/authoring/native-tone-face-a/`:
`audit.py`, `mouths.py`, `author.py`, `build.py`, `check.py`, `omissions.py` and
`after-sheets.py`. Pillow runs through the pinned Nix Python environment.
The production release CLI built each complete owned character candidate via
`build-presets`; build stdout/stderr is retained separately per character.
No GPU or game process was used.

Local evidence is in `tmp/native-tone-roster-face/group-a/`:

- `checks.json` and `<character>-checks.json`: independent component floodfill,
  inherited-contract, exact byte/pixel/metadata proof and before/after landmarks.
- `<character>-red-before.json`: observed literal previous-output failure values.
- `<character>-audit-components.json`, `*-component-summary.txt`, `*-mouths.json`
  and `*-mouths.png`: full source scan and deduplicated mouth inspection.
- `<character>-omission-review.json`: reviewed blush-color omission scan (empty).
- `after-sheet-1.png` through `after-sheet-3.png`: Vanilla, previous Player35,
  then all six actual after tones, native frame 2. `*-mouths-after.png` compares
  previous Player35 (left) with actual after Player35 (right) for every unique
  mouth pattern.
- `SHA256SUMS`: current owned source records, profiles, sets, full corpus proof
  and rendered inspection sheets.

Candidates in `generated/native-tone-roster-face/<character>/` are retained for
the parent whole-roster binding and review. These complete candidates occupy
about 1.7 GiB; the group's audit/inspection artifacts occupy about 112 MiB.
They are required evidence and integration inputs, so they are retained.

Automated preservation and source-bound correctness do not establish visual
acceptance. The parent batch owns the single consolidated user review, current
package/native-atlas integration and final acceptance status. No commit,
staging, publication or runtime/menu changes were performed by this group.
