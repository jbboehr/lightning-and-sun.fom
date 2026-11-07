# Celine native-tone portrait blush follow-up

The user reported problems on Celine's darker portrait choices, specifically
Autumn/Garden `blush` and `angry_blush`, then clarified that many expressions were
affected. This bounded follow-up corrects the reproduced pale cheek patches for
the six new native-tone choices. The previous full native-tone review is frozen;
this work does not record acceptance of that gallery or the new correction.

## Cause and authored scope

Celine's original cheek blush `#F4AEAE` was deliberately outside the historical
skin profile. It therefore stayed pale while the surrounding skin became dark.
The same issue also affects the special-blush cheek patch, which uses `#FCB0B6`
and `#E6687A`. Those two colors also occur in lips, mouth details, clothing and
other protected material, so whole-image color replacement would be incorrect.

The selected [new profile](../../palettes/profiles/celine-native-tone-blush.json)
copies the historical profile and appends three source slots, each with its own
single-color connectivity group. It adds one seed per reviewed cheek component:
334 components and 2,514 selected pixels across 63 portrait strips. All 183 Celine
portrait strips and 207 world strips remain selected. Every original source hash,
dimension, fourteen source colors, color group, region and seed is retained.
Only the six new choices map the appended slots to adapted colors. The four
historical choices use exact source-color identity targets for these slots.

The durable [cosmetic mapping and component record](celine-native-tone-blush.json)
contains source hashes, component seeds, bounds, pixel counts and all target
colors. Its base profile is
[`celine-world-trial.json`](../../palettes/profiles/celine-world-trial.json).
The selected [recipe](../../palettes/sets/celine-native-tones.json) keeps all IDs,
labels, ordering and first fourteen mappings intact.

Each target RGB channel is the original blush channel multiplied by the donor's
light channel, divided by Celine's original light `#FCDEBE`, rounded half up.
This keeps the original blush pattern and relative channel contrast while
bringing its brightness into the new skin range. These are authored cosmetic
colors, not additional sampled native ramps. The table below is ordered as
`#F4AEAE`, `#FCB0B6`, `#E6687A`; alpha remains 255.

| Choice | Blush | Special blush light | Special blush dark |
| --- | --- | --- | --- |
| Light peach (Player 04) | `#F3A9AC` | `#FBAAB4` | `#E56579` |
| Light golden (March) | `#E6AD97` | `#EEAF9E` | `#D9686A` |
| Warm peach (Wiscar) | `#DC7E71` | `#E38076` | `#CF4B4F` |
| Warm brown (Player 16) | `#D76853` | `#DE6957` | `#CB3E3A` |
| Deep brown (Player 22) | `#893133` | `#8E3236` | `#821E24` |
| Very deep (Player 35) | `#48121A` | `#4A121B` | `#440B12` |

The cheek-only component boundary excludes all other uses of these colors.
Ordinary mouth light/dark pixels, closed-mouth details, tongue, eyes, tears,
hair and clothes retain their previous output. World recoloring is unchanged.

## Focused verification

[`tests/celine_native_tone_blush.rs`](../../tests/celine_native_tone_blush.rs)
contains a regular, source-only test for profile lineage, historical mappings,
single-color groups and target arithmetic. It does not require development docs
or private game assets, so it also runs in the filtered Nix source package.

The first opt-in test applies the current recipe to both reported portraits,
special blush and neutral portraits. It checks both native frames, explicit
source blush landmarks and protected mouth landmarks. Before the correction,
the reported angry-blush cheek at `[153,77]` remained `[244,174,174,255]` instead
of the intended very-deep `[72,18,26,255]`; the regression failed for that reason.
After the correction it passes. Red and green logs are retained under
`tmp/celine-tone-followup/regression-{before,after}.log`.

The second opt-in test compares the complete freshly generated Celine candidate
against the frozen previous build. All source PNG hashes must match the original
profile. Only independently enumerated source-color components from the durable
record may change. Results for 390 strips × ten choices:

- 378 portrait PNGs change: 63 affected strips × six additions.
- Exactly 15,084 blush pixels change: 2,514 source pixels × six additions.
- All 1,560 PNGs for the previous four choices remain byte-exact.
- All 1,242 world PNGs for the six additions remain byte-exact.
- All 720 unaffected portrait PNGs for the six additions remain byte-exact.
- Every metadata file and every alpha value remain exact. All 40,279,680 decoded
  pixels in the affected images are checked; all pixels outside the selected
  cheek components remain exact.

All three dedicated tests pass, including both opt-in private-corpus tests.
The complete candidate was built with the current release CLI into the new
`generated/celine-native-tone-blush` directory. The previous
`generated/native-tone-build` output was neither rebuilt nor modified.
Commands, JSON output and logs are retained under `tmp/celine-tone-followup`.
The parent integration record owns standard repository checks and strict
packaged install/removal evidence; no GPU or new in-game run was needed for
these data-only portrait changes.

## Complete before/after review and limits

The self-contained [Celine portrait comparison](../../tmp/celine-tone-followup/review/index.html)
and [portable ZIP](../../tmp/celine-tone-followup/review.zip) expose all 183 strips,
366 native frames and six new choices. Each row reads **Vanilla → Before → After**;
Before is the exact PNG from the previous native-tone gallery. The default opens
the reported Autumn/Garden blush on Very deep. Expression/outfit search, native
frame selection, all-six display and direct links expose the remaining cases.
The full native portrait is followed by a nearest-neighbor enlarged face.

The packet copies 2,562 source-bound PNG/metadata files into 2,570 portable entries.
Every copied file is tied to its actual frozen before or fresh after build by
SHA-256, and the ZIP reproduces the complete portable directory.
The ZIP is 33,254,032 bytes, SHA-256
`012804f24120e6a738ef3e56d254fcf11de839903d198fa4641d0225ef70113f`.

Twelve CPU Chromium desktop/mobile page loads pass, each with eleven native-frame,
tone, search, outfit and all-six interaction checks. Portable hashes, ZIP contents,
all source bindings, frame metadata and all 4,758 reachable comparison frame cases
are independently checked. The evidence is in
`tmp/celine-tone-followup/review-checks.json`, with focused screenshots beside it.
Internal inspection covered the two reported examples, special blush, embarrassed,
teary and neutral portraits, plus both native frames of Spring blush, Summer
embarrassed, Winter closed-eye blush, Beach bath neutral and Wedding happy.
`tmp/celine-tone-followup/outfit-inspection.json` records those additional cases.
Automated checks and internal inspection do not establish visual acceptance.
The correction still needs the user's consolidated assessment after integration
checks finish. The previous accepted artwork and its qualifications remain in
force, and no commit or publication is performed by this follow-up.

Retained artifacts are scoped to the fresh Celine build, the portable comparison,
source/component evidence and logs. The first internal gallery packet is also
retained (it precedes a season/outfit filter-label correction); its unchanged ZIP
SHA is recorded in `review-initial-build.json`. The final ZIP above supersedes it.
The fresh build uses about 159 MiB, final gallery 60 MiB and ZIP 32 MiB; the initial
internal packet uses a further 92 MiB. The historical full gallery/build remain
available as the exact before baseline. No user data or unrelated workload is
modified.
