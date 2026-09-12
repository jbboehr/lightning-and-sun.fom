# Stillwell portraits

The profile covers all 36 main Stillwell portrait strips in the supplied
archive: nine expressions in each of Spring, Summer, Autumn and Winter. The
expressions are closed_eyes, embarrassed, happy, mad, neutral, sad, think, ugh
and wink. Every 592×180 PNG contains two 296×180 frames, with duration 0.2;
all 72 frames are unique. All four seasons use `PortraitsMisc`, and each has
its own source subfolder.

The local source archive SHA-256 is
`b4d4b47afa1459d0d4aa4b6cacffe7b2f30f11a3f6a35e471e58bb2dafc636f5`.
This pass read the verified original backup `tmp/momi-lab/assets.bak.zip`,
because the game mount was unavailable. Exported originals are under
`extracted/stillwell-portraits-study`; game images, generated variants and
review sheets remain ignored. Git contains only the recipes, source-bound
component seeds, this note and the focused test.

## Colors and boundaries

Two fine shades supplement the four catalog colors:

| Source | Target role | Source detail |
| --- | --- | --- |
| `#E8C99E` | Light | Face, ears, exposed chest, arms, wrists and fingers |
| `#CBA282` | Medium | Jaw, curled wrist crease and body shading |
| `#BD876C` | Shadow | Forehead, face and hand contours |
| `#8F564C` | Deep | Hairline, nose, neck and finger creases |
| `#5A2F2F` | Deep | Fine eye, lip, necklace-adjacent skin and finger contours |
| `#AA6B60` | Shadow | Closed/speaking lip and lower chin shading |

All four targets use the standard light/medium/shadow/deep ramp roles; no
custom blended colors are needed. Lip edges follow the skin, while the pink
speaking interior and tongue remain original. The small closed and speaking
mouths were compared in all four palettes. Original embarrassed blush and
Ugh expression details also remain unchanged.

All occurrences of these six source colors belong to exposed skin in this
corpus. There are no matching-color clothing or iris exclusions. Actual gray
eye colors (`#5E6167`, `#6C6363`, `#7F8389` and `#BABABC`), blue hair, silver
jewelry and seasonal clothes use separate colors and remain unchanged.

The selections include narrow skin gaps beside the long hair and necklaces,
the hand resting against the face, lower curled fingers and the Summer arms
and shoulder cutouts. Winter fingerless glove material stays original while
exposed fingertips, wrists and small chest/lacing gaps recolor. The profile
stores six singleton color groups and 15,887 explicit component seeds across
36 hash-bound source regions. Installation uses these reviewed selections.

## Verification

Debug Blue, Hayden, Ryis and Seridia use the same selection. Vanilla is
implicit in the installed control; shared integration assigns Stillwell F4.

```sh
nix-shell --pure --run 'cargo test --locked --test stillwell_portraits -- --ignored'
target/release/mistria-palette build-presets \
  --original extracted/stillwell-portraits-study \
  --presets palettes/sets/stillwell-portraits-trial.json \
  --output generated/stillwell-portraits-trial
target/release/mistria-palette review-batch \
  --archive tmp/momi-lab/assets.bak.zip \
  --config palettes/review/stillwell.json \
  --output generated/stillwell-review-authored
```

Use fresh output directories. Final variants are under
`generated/stillwell-portraits-reviewed/variants`. They are byte-identical to
the independently approved candidate, and all four targets passed exact
validation against the originals and metadata. Each target changes 70,118
selected pixels, with zero matching-color exclusions. No source/target pair
is an identity mapping. The common selection digest is
`3fee4a8bd30f3431e64df7f936bdc1b9b2c846412f8b51a77e7e0252777178dd`.

The ignored corpus test applies all four target ramps and checks all 36 strips
for dimensions, unchanged metadata, alpha, exact source-or-target colors,
unchanged unrelated colors and equal selections. Its 43 literal source-art
landmarks per target cover the six skin shades, hairline and lips, fine
necklace-adjacent skin, Summer arms and shoulder, Winter wrist/fingers, gray
eyes, hair, jewelry, tongue, blush and clothes. Coordinates were chosen from
the source art independently of stored component seeds.

`FOM_STILLWELL_RECIPE` overrides the Blue recipe for negative controls. The
catalog-only four-color recipe failed at the missed `#5A2F2F` finger crease
in Autumn neutral `[183,122]`. A broadened unrestricted recipe that also
classified gray `#5E6167` as skin failed at an altered closed-eye pixel
`[165,71]`. The six actual skin colors themselves need no component exclusions.
Both negative controls were observed failing, followed by a passing all-four
corpus test and a fresh passing run after formatting.

The author inspected all 72 actual full Source/Blue/Ryis trios, all 72 enlarged
face/neck trios, four seasonal body/hand views and six mouth/hand closeups in
all four targets. The independent reviewer separately checked the full corpus,
26 distinct mouth-region crops in all four targets and 34 literal landmarks.
Its 144-strip / 15,344,640-pixel audit found no remaining concrete omission or
spill. The original blush contrast remains a subjective cosmetic detail.

The authored gallery reuses all 72 frames without conflicts. Reconstructing
the profile from its selected components reproduces every actual Blue pixel
and all metadata. Corrected temporary authoring previews also match all 72
frame occurrences. Local evidence includes:

- `tmp/stillwell-test-{catalog-red,spill-red,final-green}.log`
- `tmp/stillwell-{blue,npc_hayden,npc_ryis,npc_seridia}-validation.json`
- `tmp/stillwell-actual-{full,faces,body}-*.png` and `tmp/stillwell-actual-mouths.png`
- `tmp/stillwell-roundtrip-verification.json` and `tmp/stillwell-gallery-check.json`
- `tmp/stillwell-reviewed-output-check.log` and `tmp/stillwell-final-inputs.sha256`
- `tmp/stillwell-taliferro-art-review-stillwell-verification.json`

Final profile SHA-256 is
`0abe5ccfde62304a0b1cb16dab7b7c900de092f06882e5184a7e212286074c9e`.
The preset set SHA-256 is
`5b79dc36a75ff3ce2a609848546a5b9a3642a51e6fcdf3d60029e7426fbf69bb`.

Shared integration owns the combined preview, full repository checks and
installation. Live-game checks require restoring the game executable mount.
