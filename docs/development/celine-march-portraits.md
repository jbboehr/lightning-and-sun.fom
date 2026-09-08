# Celine and March portrait batch

This continues the two-author workflow established by
[Reina and Juniper](parallel-portraits.md): one author per character, a separate
art reviewer, and one integration owner. Only recipes, source-bound masks,
configuration, tests, and documentation belong in Git. The user receives one
compact completed preview, without component selection work.

## Coverage

| Character | Spring | Summer | Autumn | Winter | Beach/bath | Wedding | Total |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Celine | 44 | 44 | 44 | 22 | 23 | 6 | 183 |
| March | 36 | 36 | 36 | 36 | 25 | 12 | 181 |

Celine includes gardening variants in Spring, Summer, and Autumn. March includes
hurt and eight-heart expressions. Overworld, UI, and child sprites remain outside
these profiles. Both characters start on Vanilla; Debug Blue, Hayden, Ryis, and
Seridia colors follow. The previous batch's possible mouth-color issues and
Hayden's embarrassed-expression refinements remain deferred.

The combined set contains 984 original strips: 967 portraits and the existing
17 Adeline world animations. Four recolors produce 3,936 variant strips.
Authoring details belong in [Celine](celine-portraits.md) and
[March](march-portraits.md).

## Integration and controls

The export command's bound increases from 143 to 256 distinct PNGs so the same
build/install path accepts both complete characters. The boundary test first
failed when requesting 256 strips; the updated exporter accepts 256 and rejects
257, empty, or duplicate selections before publication. The character package
test likewise failed on unsupported Celine before registry integration, then
passed for each new character alone and all seven together. Existing standalone
and combined cases remain covered.

Each registry atlas comes from source metadata. Celine and March beach portraits
use `PortraitsSummer`; wedding portraits use `PortraitsMisc`. Celine's garden
portraits remain within their seasonal atlas. The local preview lists them within
each season, then calls the native `spring_garden`, `summer_garden`, or
`autumn_garden` wardrobe with the unprefixed expression.

Insert cycles Celine; U cycles March. MOMI supports both names. End is unsupported
by its resolver, and Delete opens the game's debugger. U is unused in the supplied
default control bindings and direct keyboard handlers; customized controls can
still overlap prototype keys. Existing F6/F8/F10/Home/Page Down bindings remain.
A future palette menu can replace this growing list of prototype keys.

The ignored `./tmp/play-characters` launcher uses `tmp/celine-march-playtest`,
with separate state and read-only source mounts. F7 opens Celine and advances
expressions; Page Up goes back, F4 switches characters, and F5 switches outfits.
Press F1 after portrait switches before using the optional Adeline world controls,
so the preview actor resets to the reviewed Spring outfit.

## Local build

```sh
target/release/mistria-palette build-characters \
  --archive tmp/fields-of-mistria/assets.zip \
  --characters palettes/sets/characters-trial.json \
  --output generated/characters-celine-march-trial
```

The compact comparison is `generated/celine-march-preview/index.html` and its
`summary.png`. Previous trial copies remain available separately.

## Verification

Boundary and registry red/green evidence is recorded in
`tmp/celine-march-export-red.log`, `tmp/celine-march-characters-red.log`, and
`tmp/celine-march-integration-green.log`.

Both authors inspected their full frame corpora. Independent review checked all
four targets with 47 literal source landmarks per character, every decoded pixel,
source hashes, dimensions, alpha and transparent RGB, unchanged unrelated colors,
exact target mappings, metadata, and identical masks across targets. Celine's
11 shades cover pale hands and fine neck shadows while protecting hair wedges,
clothes, jewelry, and a lower-lip detail. March's 13 shades include fine body
shadows while protecting eyebrows, mouth interiors, and necklace cords. No
concrete art defect remained from that review; fine mouth and brow interpretations
still require visual judgment.

Fresh formatting, Clippy, all 87 active synthetic tests, three opt-in GML tests,
both local character corpus tests, and the release build passed. The combined
package passed exact recipe validation for all 3,936 variants. A separate registry
audit matched every new source path and atlas, profile/preset arity, standalone
Blue recipe, and catalog target roles. A native-definition check resolved all
364 new portrait names, including the 66 gardening variants.

MOMI installed the seven-character package into the isolated trial and verified
the installed pixels, animation metadata, scripts, and preserved mod selection.
Uninstalling a separate copy restored the source archive byte for byte, retaining
SHA-256 `b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
The source game remains mounted read-only in the preview.

The combined portrait atlases now occupy 16 pages versus 13 in the accepted
five-character trial. Their full RGBA8 texture capacity is 1,024 MiB versus
832 MiB; this is not measured resident memory. The preview retains Spring and
loads the atlas groups needed for the current outfit, unloading unused groups.

The real game, running with software rendering in an isolated display, checked
all 967 portrait strips and their 4,835 portrait/palette pairs across a full
session and a six-case follow-up. Checks asserted the selected sprite, fractional
animation phase, independent character selections, and a hidden debugger console.
Insert and U each completed a full cycle back to Vanilla.
Celine's gardening and March's special expressions passed the same checks.
Blue and natural-palette screenshots were inspected alongside the compact preview.

With an Adeline world actor active, the trial also passed six outfit transitions,
switching all seven characters within the beach group, and 24 action/facing
phase and movement checks. The actor was reset to Spring before those action
checks, as established in the previous batch. No script or callback failure was
logged during either session.

The first log audit found six first-Summer expressions without palette checks.
The input driver sent F7 before F9 completed; buffered keys were dispatched in
registration order, advancing the expression before checking it. The focused
follow-up waited for each exact expression and completed check. The shared local
driver now waits for label changes after navigation and a fresh exact-source
check before advancing. This timing fix changes the test driver only.

Local evidence includes `tmp/celine-march-final-checks.log`,
`tmp/celine-march-art-review-final.md`, `tmp/celine-march-integration-audit.json`,
`tmp/celine-march-native-portraits.json`, `tmp/celine-march-build-report.json`,
`tmp/celine-march-install-report.json`, `tmp/celine-march-uninstall-report.json`,
`tmp/celine-march-validation-count.txt`, `tmp/celine-march-atlas-memory.json`,
`tmp/celine-march-live-verification.txt`,
`tmp/celine-march-main-game.log`, and
`tmp/celine-march-missing-headless/state/game.log`.
The user accepted the batch visually and approved committing it. The earlier
mouth-detail follow-ups remain deferred.
