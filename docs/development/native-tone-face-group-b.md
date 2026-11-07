# Native-tone facial correction: group B

Authoring completed 2026-11-06 for the 17 characters assigned to group B.
The user reported near-universal unscaled blush and sporadic discoloration near
mouths on the full-roster expression sheet. These are proposed visual corrections;
passing the checks below does not establish artwork or live-game acceptance.

Thirteen characters have confirmed expression-only cheek or nose flush that was
missing from the six added donor mappings. Louis, Vera, Wheedle and Wynne have no
separate cheek flush in their selected source expressions. Preserve their real
glasses, eye makeup, lipstick, hair, mouth interiors and tongue colors.

| Character | Disposition | Changed source strips | Selected source pixels |
| --- | --- | ---: | ---: |
| Hemlock | Cheek flush | 4 | 136 |
| Louis | Audited unchanged | 0 | 0 |
| Luc | Flush below the glasses | 4 | 120 |
| Maple | Cheek flush; existing lip mappings retained | 4 | 128 |
| Merri | Cheek flush | 4 | 184 |
| Terithia | Nose flush and darker flush border | 4 | 160 |
| Stillwell | Visible cheek flush | 4 | 80 |
| Taliferro | Cheek flush | 4 | 136 |
| Vera | Audited unchanged | 0 | 0 |
| Wheedle | Audited unchanged | 0 | 0 |
| Zorel | Cheek flush | 4 | 176 |
| Darren | Cheek flush | 1 | 32 |
| Linnet | Cheek flush; scar retained | 1 | 26 |
| Wiscar | Cheek flush; beard retained | 1 | 30 |
| Wynne | Audited unchanged | 0 | 0 |
| Caldarus | Cheek flush and darker border | 37 | 1,406 |
| Seridia | Nose flush and Flashback Priestess outer lip | 56 | 2,027 |

The common profile and four earlier preset objects stay unchanged. Each corrected
character's six new presets select an explicit facial profile that extends the
original sources, color groups and component seeds. All existing skin-role target
values remain exact. Seventeen source-color slots are appended across the 13
profiles, with unique entries in each profile; Seridia uses 53 of the supported maximum 64 slots. The common muted-rose
ladder follows the [improved Celine treatment](celine-native-tone-face.md). Darker
Terithia and Caldarus flush borders retain their shading. Flashback Priestess lips
use the corresponding Celine outer-lip and lower-lip ladders.

Each [character record](native-tone-face-corrections/seridia.json) contains the full
new-six target tables, source archive/profile bindings, literal components with
single-color flood-fill seeds, target landmarks, protected material landmarks and
an explicit audit disposition. Sibling records cover all 17 group B characters.
No full-image RGB replacement is used.

## Mouth and special-material audit

The main-form mouth audit inspects 255 distinct source crops across the complete
654-strip portrait inventory, including both native frames and every selected
outfit/expression. Residual existing skin RGBs at mouths were checked against the
actual source materials. Louis's lip interior, Luc's iris contour, Maple's mouth
interior and Darren's beard/interior remain intentional materials. No confirmed
main-form outer-mouth skin holes were found in this group.

The fixed main-form mouth rectangle places Flashback Priestess's mouth too high.
A dedicated second audit inspects its four distinct actual mouth crops across all
seven selected strips. It confirms that `#E6687A` outer-lip pixels and `#F09DA3`
lower-lip pixels stayed bright at every donor depth. Only those lip components are
selected. A flower below the face shares `#E6687A` and remains exact, as does the
pink tongue. Seridia's record supplies explicit review crops for all 25 alternate
form portraits, including the larger Dragon portrait.

Caldarus and Seridia's permanent white under-eye markings are not blush. They
remain exact. Caldarus's stone/dragon portraits and Seridia's Dragon and spectral
Priestess portraits remain exact, along with every world strip and all spectral,
stone, glow, hair, iris, sclera, lash, tooth and mouth-interior material outside the
recorded cosmetic components. Louis's previously discussed eye band and Linnet's
scar remain as reviewed source materials.

## Verification and limits

The [literal landmark check](../../tmp/native-tone-roster-face/group-b/landmarks-before.json)
was observed to fail for all 102 expected target assertions against the frozen
before PNGs. The [same check passes](../../tmp/native-tone-roster-face/group-b/landmarks-after.json)
against the final candidates, including both appended Flashback lip colors.

The independent [corpus check](../../tmp/native-tone-roster-face/group-b/corpus-checks.json)
reconstructs each allowed component from the preserved source PNG, then checks
all 19,050 actual candidate PNGs and their metadata. It verifies 7,620 earlier-choice
PNGs and 12,510 world PNGs byte for byte. Across the six additions, only the
recorded components change: 128 portrait strips, 1,734 components, 4,641 source
pixels; 768 resulting PNGs and 27,846 changed pixels. All other decoded pixels,
alpha and metadata stay exact. The changed-image proof examines 81,838,080 decoded
pixels. The 234 protected material landmark assertions also pass.

The author viewed all six donor treatments in the four
[close-up sheets](../../tmp/native-tone-roster-face/group-b/after-all-six-0.png)
and both Flashback mouth frames in their
[dedicated comparison](../../tmp/native-tone-roster-face/group-b/seridia-flashback-all-six.png).
These inspections support the bounded selections and do not claim independent
human acceptance of all 1,308 native portrait frames. Parent integration owns the
complete review gallery, common regression tests, packaging, full installation and
removal checks. This authoring used CPU image tools and the existing release CLI;
no runtime, GML, GPU session, staging or commit was performed.

The final 17 candidates occupy approximately 837 MiB of file blocks as retained build/reproduction
artifacts. The group B audit directory also retains the intermediate Seridia
blush-only candidate (approximately 230 MiB) and source-bound audit evidence; it is
superseded by the final candidate and is not an input to integration.
