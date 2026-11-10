# Owner playthrough guide

Prepared 2026-11-08 for the owner's full playthrough with the mod active. The
playthrough is the ordinary-gameplay check for the open items in the
[overview](../planning.md#remaining-work). The [README](../../README.md) owns the
end-user install, control and removal steps; this page adds what matters for a
long test run.

## Before starting

This is the first installation on a real game copy. Every earlier check used an
isolated copy of the game archive, with no real saves.

1. Turn off automatic game updates. The installer replaces `assets.zip`, and the
   README requires removing the mod before any game update.
2. Back up the game's save folder and copy `assets.zip` from the game directory.
   Removal restores the exact archive, but the copies cost little and protect
   against anything unexpected.
3. Confirm `sha256sum assets.zip` matches the fingerprint in the
   [compatibility section](../../README.md#compatibility). A different value
   means a different game build than the one tested; stop and report it rather
   than using `--allow-source-hash-mismatch`.
4. Build and install the complete collection, as in
   [Build and install](../../README.md#build-and-install). The install command
   should print `Installing the complete collection from` followed by a path
   inside the Nix store. Keep the `result` link and the `.mistria-palette/`
   recovery directory until removal.

## During play

Turn on **Remember choices** so selections survive restarts; a long run is the
first real test of that storage. Give characters non-Vanilla palettes, ideally
contrasting ones such as Debug Blue or Very deep, so a missed frame stands out.
Changing choices partway through also exercises the menu.

Watch for:

- skin left in its original color on some frames, especially at the start or end
  of an action, which is where the untested animation end stage would show;
- clothing, hair, scars or markings recolored as if they were skin;
- a character's portrait and world sprite disagreeing;
- palettes reverting to Vanilla after a restart while remembering is on, or the
  warning that saving settings failed;
- any game error message, especially one mentioning Adeline or pathfinding;
- if you play with a physical controller, anything wrong when navigating
  **Settings → Palettes**.

Report each problem with the date in game, the character, the palette, and what
was happening. Copy the exact text of any error. Tell the agent directly or note
problems in `tmp/playthrough/notes.md`; screenshots are useful but stay local,
since they contain game-derived images.

## Afterwards

Close the game and uninstall as in
[Remove or change the mod](../../README.md#remove-or-change-the-mod) before
updating the game or using MOMI separately. Removal restores the exact archive
from before installation.
