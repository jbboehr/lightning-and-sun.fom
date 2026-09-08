# Eiland and Olric portrait batch

This continues the [parallel portrait workflow](balor-valen-portraits.md): two
character authors, an independent art reviewer, and one integration owner.
The user receives a compact comparison of completed recolors. Game images,
previews, extracted files, and installable packages remain under ignored paths.
Earlier mouth-detail follow-ups for other characters remain deferred.

The later [Eiland eye, cheek, and collar correction](eiland-portraits.md#eye-cheek-and-collar-follow-up)
and [Olric eye correction](olric-portraits.md#eye-and-eyebrow-follow-up)
record the user's visual feedback and the current profiles, preview, and trial.
The initial batch evidence below predates those corrections.
The user accepted the corrected combined summary and approved committing the batch.

## Coverage

| Character | Spring | Summer | Autumn | Winter | Beach/bath | Wedding | Total |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Eiland | 14 | 14 | 14 | 14 | 15 | 7 | 78 |
| Olric | 9 | 9 | 9 | 9 | 0 | 0 | 36 |

These are all main portrait strips in the supplied archive, including Olric's
four seasonal bunny-ears neutral portraits. Eiland's beach/bathing portraits use
`PortraitsSummer`; wedding uses `PortraitsMisc`. Olric's portraits use the four
seasonal atlases. Overworld, UI, and child artwork remain outside these profiles.
The combined eleven-character trial contains 1,300 original strips: 1,283
portraits and the existing 17 Adeline world animations. Four targets produce
5,200 variants. Vanilla remains the default, followed by Debug Blue and three
sampled NPC palettes. Character authoring is recorded in
[Eiland](eiland-portraits.md) and [Olric](olric-portraits.md).

## Integration and preview

J cycles Eiland; K cycles Olric. MOMI's resolver accepts these letter keys, and
neither is used by the supplied default bindings or direct keyboard handlers.
Existing character controls remain available. Shared production Rust and GML
behavior does not change for this data-only addition.

All 114 source names were checked against native NPC definitions. Olric's
`spring_bunny_ears_neutral` and its seasonal counterparts use the outfit
`spring_bunny_ears` with expression `neutral`. The local trial helper translates
these names when constructing speakers, as it already does for Celine's gardening
and Juniper's beach-accident outfits. The production registry retains the exact
original animation names and atlases.

The local helper's F5 skips missing outfit groups. Olric cycles the four seasons;
Eiland has all six groups. F4 keeps a supported outfit or falls back to spring.
An interpreter regression first failed when the old helper tried to display
Olric's nonexistent beach portrait, then passed four/six-outfit cycles, character
fallback, shared-outfit preservation, and expression resets after the correction.
This helper is excluded from the player package.

```sh
target/release/mistria-palette build-characters \
  --archive tmp/fields-of-mistria/assets.zip \
  --characters palettes/sets/characters-trial.json \
  --output generated/characters-eiland-olric-trial
```

The combined preview is `generated/eiland-olric-preview/index.html` and its
`summary.png`, with one row per character and all five palette choices. The
isolated trial uses `tmp/eiland-olric-playtest`, separate state, and read-only
mounts of the supplied game. F7 opens Eiland and advances expressions, Page Up
goes back, F4 switches character, and F5 changes outfits. For optional Adeline
world checks, F1 resets the reviewed Spring actor before F3/F12/F11.

## Verification

The synthetic character bundle test first failed on unsupported Eiland. After
adding the registry entries, all four character integration tests passed,
including both new characters alone and the full eleven-character collection.
Native path checks covered all 78 Eiland and 36 Olric strips, including the four
bunny-ears outfit translations. The registry audit also compared each author's
extracted PNG byte-for-byte with the ZIP entry, checked profile source hashes and
dimensions, and matched source paths, atlas metadata, preset arity, standalone
Blue colors, and sampled NPC roles. The previous nine registry entries are
unchanged.

Both authors inspected every unique frame and reconstructed their exported
profiles from the authored galleries. Independent review passed all 456 new
target strips against every pixel and 91 literal source landmarks: 50 for Eiland
and 41 for Olric. All 228 unique frames and enlarged final source/Blue/Ryis faces
were inspected. Eiland's alternate hand shades and bare bathing ear recolor while
shared-color gold jewelry remains original. Olric's fine skin and lighter lip
shading recolor while eyebrow hair and dark mouth interiors stay original.
Every target preserves alpha, transparent RGB, metadata, unrelated colors, and
the same per-character changed mask.

Formatting, Clippy, all 87 active synthetic tests, both new character corpus
tests, three opt-in GML tests, and the release build passed. The focused author
tests first failed on omitted skin colors or incorrectly selected protected
details before passing with the reviewed masks. The final combined build passed
exact recipe validation for all 5,200 variants, and the previous nine characters'
complete output trees are byte-identical to the accepted Balor-mouth build.
Build input hashes remained stable through verification.

MOMI installed the combined package into the isolated trial and verified installed
pixels, animation metadata, generated scripts, and mod selection. Uninstalling a
separate copy restored the supplied archive byte-for-byte, retaining SHA-256
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
Portrait atlases use 20 pages versus 17 in the nine-character trial. Their full
RGBA8 capacity is 1,280 MiB versus 1,088 MiB; this is not measured resident memory.
The local helper retains Spring and loads the groups needed by the current
portrait outfit.

The final in-game matrix passed all five choices for every new portrait, plus
one portrait per outfit for the previous nine characters: 168 source strips and
840 distinct source/palette pairs. J/K completed full keyboard cycles, all four
Olric bunny-ears portraits rendered, and independent choices and animation phase
checks passed. Missing-outfit fallback, Olric's four-outfit wrap, and all ten new
character outfit transitions passed with the Adeline world actor present. A
separate fresh session passed 24 world phase and movement checks. Neither session
reported script errors, disabled palette handling, or console interference.
Captured Eiland and Olric portraits on the Ryis palette were visually inspected.
Later user feedback, corrections, and acceptance are recorded in the linked
character follow-ups above.

Local evidence includes `tmp/eiland-olric-integration-red.log`,
`tmp/eiland-olric-navigation-red.log`, `tmp/eiland-olric-navigation-green.log`,
`tmp/eiland-olric-native-portraits.json`, `tmp/eiland-olric-integration-audit.json`,
`tmp/eiland-olric-art-review-final.md`, `tmp/eiland-olric-final-checks.log`,
`tmp/eiland-olric-build-inputs.sha256`, `tmp/eiland-olric-build-report.json`,
`tmp/eiland-olric-validation-count.txt`,
`tmp/eiland-olric-prior-output-verification.txt`,
`tmp/eiland-olric-install-report.json`, `tmp/eiland-olric-uninstall-report.json`,
`tmp/eiland-olric-atlas-memory.json`, `tmp/eiland-olric-live-verification.txt`,
`tmp/eiland-olric-live-run.log`, and `tmp/eiland-olric-world-run.log`.
