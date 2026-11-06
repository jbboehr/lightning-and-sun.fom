# Hayden embarrassed-expression follow-up

2026-11-05. The source/mask audit is complete. **No concrete mask defect was
isolated; no artwork was changed. The user subsequently accepted the displayed
details, with inspection depth unspecified (see the assessment below).** The
historical report described a few odd pixels without exact positions. This audit
narrows the visible warm details to named source materials but does not claim to
have identified what the user originally noticed.

The [complete comparison](../../tmp/hayden-embarrassed-review/index.html) covers
all six embarrassed-expression strips: Spring, Summer, Autumn, Winter, Beach and
Wedding. Both native frames appear in full, with enlarged face and mouth/blush
crops, in Vanilla, Debug Blue, Adeline, Ryis and Seridia palettes. Hayden's current
natural alternative is Adeline; labels follow the actual configured preset IDs.
This is a complete twelve-frame, sixty-frame/palette audit, rather than a sample.

The nose/cheek stripe is original `#E88671` blush, 54 pixels per strip. It remains
unchanged in all choices. The open mouth retains its red `#A1121D`, pink
`#D4546F` and tongue `#F96F8C`; the closed mouth's selected surrounding skin already
follows the palette. Brown beard/head hair and pale sideburn streak remain hair.
No newly omitted normal skin shade or clothing leak was established in the
source/current enlarged views. Altering these expression colors without a
specific reported defect would change the existing art treatment speculatively.

## Verification and reproduction

The 133 historical portrait regions are exactly present in the selected
`hayden-world-trial.json`; their first eleven source colors and palette role
mappings agree. All 266 source PNG/metadata members match the preserved archive
`8354ff03cab354e9d12903694efac6075219034d7b7b8fd8ded24e4a04fbe0b4`.
No accepted fixture, profile, palette or source image was modified.

A fresh four-preset portrait build generated 532 variants. Exact CLI validation
passes for each preset; metadata is preserved. The existing ignored
`hayden_portraits` material test passes, covering skin shading, mouth edges,
body hair and protected hair/clothing. Literal source checks additionally cover
blush, red mouth interior, tongue and recolored forehead in every submitted
expression and palette. Every one of the 324 blush pixels across the six strips
is retained in each choice.

The gallery binds 10,158,720 preview pixels to actual output images, contains
sixty complete native-frame views with no crop loss, and pins the selected
profile/set and historical portrait profile. Fourteen CPU Chromium loads cover
all seven pages at desktop/mobile widths; all links/images load with no
horizontal overflow. Static checks do not establish user acceptance or live
rendering/input behavior. No local GPU or model inference was used.

Reproduction uses the retained helpers and evidence in
`local/authoring/hayden-embarrassed/`: `review.rs`, `browser.mjs`,
`source-bindings.json`, `prior-portraits-set.json`, `recipe-*.json`,
`validation-*.json`, `retained-tests.log`, `review-check.log` and
`browser-check.json`. The set derives only the unchanged portrait subset and its
first eleven configured roles from the selected complete profile. Original and
current face color locations are in `face-colors.json`.

```sh
target/release/mistria-palette build-presets \
  --original extracted/hayden-portraits-study \
  --presets local/authoring/hayden-embarrassed/prior-portraits-set.json \
  --output generated/hayden-embarrassed-audit
nix-shell --pure --run 'cargo test --release --locked --test hayden_portraits -- --ignored --nocapture'
node local/authoring/hayden-embarrassed/browser.mjs
```

Use a fresh build output when reproducing. The gallery helper uses the project's native Rust dependencies and retained
release build; all game images and derived outputs remain ignored. The older [Hayden authoring record](hayden-portraits.md) remains the
historical acceptance record. A later specific pixel report or visual decision
can refine this audit; passing checks alone does not close the subjective report.

After the combined second-batch review, the user identified Reina mouth issues
and no other issues. Hayden's retained displayed expression details are accepted,
with inspection depth unspecified; no live-game acceptance is inferred.
