# Celine mouth, eye and visible-blush follow-up

The user described the first blush correction as "much better" but reported
remaining problems around the mouth and eyes. They identified the nose patch as
blush that should be retained, possibly in another shade. That is qualified
feedback on the first correction, not final acceptance of Celine or the complete
native-tone expansion.

This bounded follow-up corrects source-bound facial edges for the six new choices
and restores visibly colored, muted rose blush. The original four palette choices,
all world outputs, original game images and all previous review/check/build packets
remain intact. No commit or publication is authorized by this follow-up.

## Reproduced defects and selected material

The old skin profile intentionally excluded small components around mouth and eye
features. Their warm source colors remained visible against the darker choices.
Closed eyelids also contained several source colors absent from that profile.
Open-mouth frames exposed additional pale skin corners below and beside the mouth.
The first ratio-scaled blush became nearly invisible on Player 35.

The new [facial profile](../../palettes/profiles/celine-native-tone-face.json)
preserves all seventeen previous source slots, groups, region bindings, dimensions,
source hashes and seeds. It adds 5,548 explicitly selected components containing
6,262 source pixels across all 183 portrait strips:

| Material | Components | Pixels |
| --- | ---: | ---: |
| Eye skin contours | 2,282 | 2,426 |
| Closed eyelids | 1,686 | 1,866 |
| Inner eye skin accent | 66 | 66 |
| Mouth skin contours | 1,049 | 1,105 |
| Outer lips | 465 | 799 |

The existing fourteen skin target colors remain exact. The new profile selects
previously omitted components of existing colors where they belong to facial skin.
Six additional source slots use singleton connectivity groups and these existing
skin roles:

| Source color | Material | Target role |
| --- | --- | --- |
| `#C77146` | Closed eyelid creases | Shade 2 |
| `#742F23` | Closed eyelid rims | Shade 3 |
| `#DC8E4F` | Inner eye and mouth skin accent | Shade 2 |
| `#F3AC87` | Open-mouth lower skin shade | Shade 1 |
| `#FFE1C5` | Open-mouth lower skin highlight | Shade 0 |
| `#BA5E3F` | Open-mouth skin contour | Shade 2 |

The shared recipe still selects the previous
[seventeen-slot blush profile](../../palettes/profiles/celine-native-tone-blush.json)
for the original four choices. Each of the six new choices explicitly selects the
new twenty-three-slot profile. This source-bound per-preset profile override is
implemented and tested separately in the preset loader; it prevents new facial
seeds from changing accepted historical output. IDs, labels, order and the
original fourteen mappings remain intact.

A durable [mapping and component record](celine-native-tone-face.json) contains
all source hashes, component seeds, bounds, color counts, semantic roles and full
per-choice twenty-three-color target tables. Root authoring regenerates that
overlay after the unchanged historical fourteen-slot mappings. No runtime material
classifier or whole-image color replacement is added.

Hair and brows, black lashes, iris colors, eye whites, tears, tongue and mouth
interior retain their previous pixels. Their shared RGB values remain excluded
when their connected components fall outside the selected facial material.
For example, `#B35A35` at the lower lip edge changes while a source-bound hair/brow
landmark of that same color stays exact. `#DC8E4F` at the mouth side changes while
the same color in forehead hair stays exact. Intrinsic mouth colors `#A5462C`,
`#772E24`, `#A30A17`, `#FC708D`, `#F48C9B` and `#FFABCC` are not added to the palette.

## Visible muted-rose blush and lips

The three existing cosmetic slots retain their original component shapes. Their
revised colors bring back visible blush on deep skin, without the original pale
stripe. The same two special-blush colors are applied to selected outer lips,
while tongue and interior remain separate.

| Choice | Blush `#F4AEAE` | Light blush/lip `#FCB0B6` | Dark blush/lip `#E6687A` |
| --- | --- | --- | --- |
| Light peach (Player 04) | `#EAA092` | `#F1AAA2` | `#D05E67` |
| Light golden (March) | `#DCA07B` | `#E6AA8A` | `#BE6258` |
| Warm peach (Wiscar) | `#C98476` | `#D48E84` | `#B9575F` |
| Warm brown (Player 16) | `#B85F55` | `#C66E65` | `#A2474A` |
| Deep brown (Player 22) | `#A35353` | `#B26363` | `#84363F` |
| Very deep (Player 35) | `#84434D` | `#96535F` | `#6E2F3D` |

These are authored cosmetic candidates, not additional native donor ramps.
User assessment is still needed for the shade and appearance. The original source
blush pattern is retained rather than removed or painted over.

## Focused tests and complete pixel comparison

[`tests/celine_native_tone_face.rs`](../../tests/celine_native_tone_face.rs)
contains one regular test for the shared historical profile, explicit new-choice
overrides, retained skin roles, profile lineage and literal cosmetic targets.
It reads only palette inputs, so it runs in the filtered Nix source package.

The first opt-in test applies the selected deep-tone recipe to source-bound
reported, neutral, closed-eye and special-blush portraits. A failing-before run
against the frozen seventeen-slot recipe reports thirteen independent defects:
mouth edges, pale outer lips, open and closed eye contours, open-mouth skin
highlights and almost invisible nose blush. All thirteen landmarks pass afterward.
Literal protected landmarks check hair/brows, lashes, iris, white eyes, pink/dark
tongue and mouth interior. The source hashes remain enforced by the real CLI.

The second opt-in test compares the whole freshly built candidate with the frozen
first blush correction. Reviewed components are independently reconstructed from
the original source. Across all 390 strips and ten choices:

- 1,098 new-choice portrait PNGs change: all 183 portraits × six additions.
- Exactly 52,656 facial pixels change: 6,262 newly selected source pixels plus
  2,514 existing blush pixels, multiplied by six choices.
- All 1,560 historical-choice PNGs remain byte-exact.
- All 1,242 world PNGs for the new choices remain byte-exact; combined with the
  historical choices this preserves all 2,070 world PNGs.
- Metadata and alpha remain exact. All 117,002,880 decoded pixels across the
  changed PNGs are checked, and every unrelated pixel remains exact, including
  already recolored skin outside the new facial selection.

All three focused tests pass. The fresh candidate is
`generated/celine-native-tone-face`; its before baseline is
`generated/celine-native-tone-blush`. The original full native-tone build remains
unchanged. Logs and JSON evidence are retained in `tmp/celine-face-followup`.
The parent integration record owns full standard/Nix checks and strict packaged
installation, native reconstruction verification and exact removal. This facial
authoring/review uses CPU rendering; no local GPU or inference is needed.

## Complete review and limits

The [complete portrait review](../../tmp/celine-face-followup/review/index.html)
and [portable ZIP](../../tmp/celine-face-followup/review.zip) show **Vanilla →
Before → After**. Before is the exact first Celine blush correction; After fixes
selected facial contours and shows the revised visible rose blush. The default
opens Autumn/Garden blush on Very deep. All 183 strips, both native frames and all
six additions remain reachable through portrait, expression/outfit, frame and tone
controls; the all-six option shows every new choice together.

The packet contains 2,562 source-bound PNG/metadata copies and 2,570 portable
entries, exposing 4,758 comparison frame cases. Every copied file is tied to the
actual original/before/after build by SHA-256. Portable manifests and ZIP contents
are independently validated. Twelve CPU Chromium desktop/mobile page loads pass,
each with eleven frame, tone, search, outfit and all-six interaction checks. The
ZIP is 33,277,273 bytes, SHA-256
`4718f22df5027c04f4b59f27627fc838b3ffded626119fed905424dc0f4dcb7d`.
Detailed results are in `tmp/celine-face-followup/review-checks.json`.

Internal inspection covers the two original reported examples, neutral,
closed/open eyes, special blush, teary, very happy and all four seasonal plus
Beach/Wedding outfits, in both native frames; all six additions are represented.
The selected examples and screenshot hashes are recorded in
`tmp/celine-face-followup/internal-inspection.json`.
Automated correctness and internal inspection do not establish visual acceptance.
One consolidated user review follows complete parent integration checks.

Prior frozen review/source/check packets are preserved. The fresh face candidate,
portable review, private source/component evidence and inspection captures are
retained for review and reproduction. An initial twenty-slot internal candidate
is also retained; the final twenty-three-slot candidate additionally covers the
source-bound open-mouth skin highlights discovered during the complete expression
inspection. Neither changes the preserved original archive or user data.
