# Vera portraits

The profile covers all 32 main Vera portrait strips in the supplied archive:
embarrassed, happy, mad, neutral, sad, think, ugh and wink in each of Spring,
Summer, Autumn and Winter. Every 592×180 strip has two 296×180 frames with
duration 0.2; all 64 frames are unique. Every season uses `PortraitsMisc`.

The source archive SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
This pass read `tmp/momi-lab/assets.bak.zip` because the game mount was empty.
Exported originals are under `extracted/vera-portraits-study`. Source images,
generated variants, galleries and temporary helpers remain ignored; the
repository contains only recipe data, source-bound seeds, tests and notes.

## Colors and boundaries

Two small skin shades supplement the four catalog samples:

| Source | Target role | Detail | Selected pixels |
| --- | --- | --- | ---: |
| `#B68454` | Light | Face, ears, neck, shoulders, arms and hands | 91,360 |
| `#A15F3F` | Medium | Skin shading and hand creases | 16,340 |
| `#7F402B` | Shadow | Hairline, face and body contours | 7,696 |
| `#541C1E` | Deep | Fine eyebrow/eye, nose, jaw and finger contours | 3,432 |
| `#B07342` | Medium | Small eyebrow and nose skin transitions | 224 |
| `#762E21` | Deep | One outer speaking-mouth transition per strip | 32 |

All four targets use the standard light/medium/shadow/deep ramp roles; no
custom interpolated shades are needed. All occurrences of these six colors
belong to selected skin or its edge shading in this corpus. There are no
matching-color hair, jewelry or clothing exclusions. The profile stores six
singleton groups and 18,752 component seeds across 32 hash-bound source regions.

Magenta hair and nail polish (`#581B46`, `#8F295A`), black eyebrow cores,
purple irises (`#5237A9`), pale eyes and gold earrings (`#E9AF68`) remain
original. Pink/purple lipstick (`#AB4561`, `#731A45`) and speaking-mouth
interiors (`#581D1F`, `#941A33`, `#CA4D67`) also remain original. The Ugh
expression's blue-gray marks (`#657B9F`) are preserved as expression artwork.

The brown `#762E21` mouth pixel sits outside the lipstick at frame-local
`[147,86]` or `[147,87]`, depending on expression. Both author and independent
reviewer identified the warm fringe against Debug Blue. Including it changes
exactly one additional pixel in each speaking frame; the final four outputs
otherwise match the earlier five-color candidates pixel for pixel. Original
lipstick contrast against each target skin remains a subjective cosmetic choice.

Seasonal selections include both exposed Summer arms and the small shoulder
cutouts in Winter, as well as the fingers near the mouth and hand on the hip.
Aprons, sleeves, hairdressing tools, bracelet and nail polish remain unchanged.

## Verification

Final outputs are under `generated/vera-portraits-reviewed/variants`. All four
targets built and passed exact validation against the originals and metadata.
Each changes 119,084 pixels, with zero matching-color exclusions and no identity
mappings. The selection digest, hashing each ordered path, a NUL separator and
the per-pixel change bits, is
`9a0c73e0c4f4c2a983b828c8b6b7cfb45527d269e381a515cedfa44a13cfd898`.

```sh
nix-shell --pure --run 'cargo test --locked --test vera_portraits -- --ignored'
target/release/mistria-palette build-presets \
  --original extracted/vera-portraits-study \
  --presets palettes/sets/vera-portraits-trial.json \
  --output generated/vera-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/momi-lab/assets.bak.zip \
  --config palettes/review/vera.json \
  --output generated/vera-review-authored
```

Use fresh output directories. The ignored corpus test applies all four target
ramps to all 32 strips, checking dimensions, metadata, alpha, exact replacement
colors, unchanged unrelated colors and identical selections across targets.
Its 53 literal skin and 41 protected landmarks per target come from independent
source-art inspection and cover the fine eyebrow and mouth blends, Summer arms,
Winter shoulder, hair, lipstick, nail polish, eyes, earrings and seasonal clothes.

`FOM_VERA_RECIPE` overrides the Blue recipe for negative controls. Before adding
the sixth shade, the five-color candidate failed at the missed speaking-mouth
transition in Autumn neutral `[443,86]`. An unrestricted recipe that also
classified magenta `#581B46` as skin failed at an altered hair pixel `[153,41]`.
Both controls were observed failing, followed by a passing all-four test and
another passing run after formatting.

The author inspected all 64 actual full Source/Blue/Ryis trios, all 64 enlarged
face trios, four seasonal body/hand trios and six mouth closeups in all four
targets. After the single-pixel mouth correction, every final face was inspected
again, with all-four mouth comparisons and an exact audit proving that only
the 32 intended mouth pixels changed. The full-art crop `[60,30,176,150]`
contains every visible source pixel; the corpus bounds are inclusive
`[96,40,200,179]`. The smaller face-detail crop is `[124,49,53,51]`.

The final gallery reuses all 64 frames without conflicts. Reconstructing a
profile from its selected components and applying it reproduces every final
Blue PNG and metadata file byte for byte. Local evidence includes:

- `tmp/vera-test-{mouth-red,spill-red,final-green,formatted-green}.log`
- `tmp/vera-{blue,npc_hayden,npc_ryis,npc_seridia}-validation.json`
- `tmp/vera-final-verification.json` and `tmp/vera-mouth-components.json`
- `tmp/vera-actual-{full,faces,body}-*.png` and `tmp/vera-actual-mouths.png`
- `tmp/vera-final-gallery-report.json` and `tmp/vera-gallery-roundtrip-compare.log`
- `tmp/vera-final-inputs.sha256`

Final profile SHA-256 is
`e5e8bd80757a0b059093b1c26bed4b573f58cd10857a59dda8dd7501905d65dd`;
the preset set SHA-256 is
`78b1ff35bca2d217e4aedfe75d570e020b4d793ae346ae4db6f853805c26f051`.

Shared integration owns the combined package, complete Vanilla/Debug Blue
review pages, full repository checks and the independent review report. Vera's
palette control is G. Live gameplay has not been exercised in this authoring
pass; the user accepted the offline preview.
