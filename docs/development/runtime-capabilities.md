# Runtime integration investigation

Investigated 2026-09-06 against the supplied Linux build. The initial findings below
come from source inspection and one offline pixel experiment. The subsequent
[portrait toggle](portrait-toggle.md) runs in-game using installed animations.

| Capability | Finding | Consequence |
| --- | --- | --- |
| Start a Rust helper from GML | No supported entry point identified | Do not promise that dropping an executable into a mod makes it callable |
| Create images during play | Shipped date-photo code calls `animation_decompress`, draws the result, and frees it with `animation_remove` | There is a concrete engine route to investigate, but Rust-generated input is unverified |
| Recolor the original portrait through its existing LUT support | The shader indexes colors by red alone, and the test portrait has conflicting red values | A direct LUT assignment cannot reproduce our exact RGBA mapping |

## Evidence and scope

The archive fingerprint and original portrait are recorded in
[the asset investigation](../INVESTIGATION.md). The executable SHA-256 is
`1a9057648dfec94988226a4c08d83b13b85e53e393d0e72d0eb58418a34a5b82`.

Inspected all 567 shipped GML files, relevant embedded shader text and API names,
the [official SDK](https://github.com/NPC-Studio/mistria-sdk/tree/c7a92ffdbc6f8088c6c273ba6af2250931c3d208),
and MOMI source at `dcda6dac4746dfb271d7ffb6cb75b73d8aaf19b4`. That MOMI source
revision is separate from the v0.15.10 binary used in the earlier install test.
Extracted scripts, the analysis program, and downloaded source stay in ignored
`tmp/runtime-capabilities/`. No game files or images are included in this note.

## Calling Rust

Searches of game scripts, SDK and MMAPI material, and executable strings found
none of the usual `execute_program`, `execute_shell`, `external_define`, or
`external_call` entry points. Embedded names identify file, buffer, environment,
and clipboard operations in the game's OS interface, but no process launcher was
identified. These searches are evidence about the inspected surface, not proof
that no other interface exists.

[Fabricator](https://github.com/kyren/fabricator/blob/f2483a61d53c730020f2357c790603dd6362476b/README.md),
the GML implementation used by Mistria, does provide Rust interoperability.
Its generic GameMaker project runner also has a
[native extension loader](https://github.com/kyren/fabricator/blob/f2483a61d53c730020f2357c790603dd6362476b/crates/fabricator/src/ffi.rs).
That loader consumes extension declarations from a GameMaker project. We found
no corresponding mod registration route in Mistria's SDK, `Maybe.toml`, or MOMI
manifests. Support in the generic runner does not establish support in Mistria's
custom runner. A Rust library remains conditional on such an integration point.

There is an existing external-helper pattern in MOMI:
[GameRestartMonitor](https://github.com/Garethp/Mods-of-Mistria-Installer/blob/dcda6dac4746dfb271d7ffb6cb75b73d8aaf19b4/ModsOfMistriaInstallerLib/GameRestartMonitor.cs)
watches for a request file and starts the game from the desktop process. Its
inspected implementation targets `FieldsOfMistria.exe` and waits a fixed two
seconds. This is a useful architectural example, not a verified Linux restart
solution or evidence that GML can start a process. An equivalent Rust companion
would need to be started by an installer or launcher first.

## Runtime images

The shipped `assets/gml/scripts/Dates.gml` supplies a concrete example:

- `save_date_photo` compresses a generated image and stores the result with the
  photo's data.
- `spawn_date_photo` recreates the image with `animation_decompress`, providing
  the saved data and explicit dimensions, then uses it in the popup.
- Cleanup releases runtime images through `animation_remove`.

This establishes that the game has a dynamic image path. It does not establish
that the decoder accepts PNG files or that we know its serialized format. We
have not supplied Rust-generated data, checked its alpha handling, or verified
animation frames and origins for portraits through this path.

`portrait_atlas_load` and `portrait_atlas_unload` also appear in seasonal and
cutscene code. Those calls manage named game atlases. They do not demonstrate
re-reading a modified archive or importing arbitrary files. Keep archive
installation outside the running game.

The community's [Dynamic Object Sprites rules](https://www.nexusmods.com/fieldsofmistria/articles/88)
switch between sprites that must already be installed when rules are registered.
That feature alone does not solve runtime import either.

## The built-in palette shader has a specific limit

`UI/Anchor/Menus/TextboxMenu.gml` creates portraits as Anchor sprite nodes.
`UI/Anchor/Node.gml` gives these nodes `set_lut`, `set_lut_index`, and
`disable_lut`. LUT images must be 256 pixels tall. `UI/Anchor/Anchor.gml` binds
the lookup texture during each affected node's draw and resets the shader mode
afterward.

However, the embedded `ANCHOR_SPRITE` shader selects its lookup row using the
red component of the tinted source pixel. The separate `PALETTE_SWAP` mode
also indexes by red. Neither performs arbitrary RGB equality matching.

A temporary Rust program decoded our original and stylized Adeline portrait,
grouped opaque pixels by original red value, and counted required output RGB
colors in each group:

- 32 distinct original opaque RGB colors use only 27 distinct red values.
- Five red values need more than one output color.
- Even the best possible single red-indexed lookup must get at least 1,662
  opaque pixels wrong, with normal white tint.
- The same obstruction exists when the desired output is the original image.

The lower bound sums each group's pixel count minus its most frequent required
output color. It proves the direct shader shortcut cannot preserve this portrait.
It is not a measurement from a GPU render.

One possible later design is to have Rust prepare indexed base images and LUTs
locally during installation, then let GML select presets during play. This would
still require local preparation and installation of game-derived images, but
would not require distributing them. It is an alternative to investigate, not
an implemented change to the current exact-color pipeline.

## Chosen next slice

Generate the alternate portrait locally with Rust during installation, install it
alongside vanilla through MOMI, and use GML to switch the displayed animation.
The [one-portrait prototype](portrait-toggle.md) implements this route. It does not
require a process launcher or an image decoder during play, and its source package
contains no game-derived images.

Defer the date-photo decoder experiment. If revisited, first establish its format
with a synthetic image, alpha, dimensions, repeated replacement, and cleanup.
A process-launch or native-extension route still needs separate evidence before
choosing an architecture that starts Rust from inside the mod.

MOMI's [GML compile checker](https://github.com/Garethp/Mods-of-Mistria-Installer/blob/dcda6dac4746dfb271d7ffb6cb75b73d8aaf19b4/ModsOfMistriaInstallerLib/Tools/GmlCompileGate.cs)
does not reject unresolved identifiers. Passing a script with a guessed API name
through that checker would not verify the capability. Actual engine execution
is still needed for the proposed runtime-image experiment.
