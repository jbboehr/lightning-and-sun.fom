# Hayden's ground-shadow sprites

The 29 PNG strips under `assets/animations/NPCs/Hayden/Sprites/Shadow/` are
excluded from skin recoloring. Direct pixel inspection found 127 frames:
53 fully transparent and 74 containing ground-shadow silhouettes. Eleven entire
strips are transparent. Every one of the 11,512 visible pixels is `#252B43FF`;
there are no intermediate alpha values or skin colors. The nonempty artwork
forms flat ovals beneath riding poses. All frames are 80×80 on the Default
atlas, with Middle/54 origin or its numeric horizontal equivalent 40.

The source archive is read-only `tmp/fields-of-mistria/assets.zip`, SHA-256
`0544f78f8f410320c5286b9955d67bf58e5d94ef8c25b9d6a67e38f9291f02e4`.
Its native files establish a separate ground-shadow rendering path:

- `assets/gml/scripts/GameplaySystems/ShadowDictionary.gml` loads the animation
  shadow manifest and looks up a shadow animation for the character sprite.
- `assets/data_files/animation/shadow_manifest.json` has 52 mappings targeting
  28 of these strips, including seasonal tools, seated reading and Spring riding.
- `assets/gml/scripts/ShadowCasters.gml` creates separate shadow renderables.
- `assets/gml/objects/characters/obj_hayden.gml` updates the shadow lookup during
  the farm introduction.
- `assets/gml/objects/system/obj_shadow_level.gml` draws the shadow collection
  with global shadow color/alpha and multiplicative blending.

The fully transparent `spr_npc_hayden_specialanimation_shadow_ride_south` is
absent from that manifest; this does not rule out other native references.
None of these assets needs a palette variant, mask or registry entry. Their
native PNGs, metadata and mappings remain untouched.

Ignored evidence: `tmp/hayden-shadow-audit-pixels.json` records every pixel-color
count, alpha count, per-frame visible count and raw sidecar; the summary,
mapping report and targeted native source copies share that prefix. The
[first-frame contact sheet](../../tmp/hayden-shadow-audit-contact-sheet.png)
shows one frame per strip. All frames, including those outside the sheet, were
included in the pixel audit. The exported sources are in
`extracted/hayden-shadow-audit`. No live rendering was tested by this audit.
