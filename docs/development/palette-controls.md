# Palette controls and remembered choices

Implemented on 2026-11-05 against the preserved game archive and pinned MOMI
v0.16.4. CPU interpreter, native engine API/IO and rendered title/journal UI checks pass.
Actual keyboard/mouse and virtual gamepad are exercised; **the displayed live
review has qualified user acceptance; physical-controller acceptance remains pending**. These checks do not
establish additional artwork acceptance. See the [integrated batch record](finish-batch-2026-11-05.md) for native phase evidence and display limits.

## User behavior

The native Settings screen gains one **Palettes** category. Each of the 36
character rows shows its current palette. Selecting a row cycles that character's
available choices, including Vanilla. The row updates in place, retaining its
pilot and scroller. Native Settings navigation owns back, cancel and close.
There is no new gameplay binding. Experimental character hotkeys register only
when the QA fixture explicitly sets `global.__lns_palette_debug_hotkeys = true`.

**Remember choices: Off** is the default. Without opting in, every launch begins
with Vanilla and changing palettes or resetting writes no configuration file.
Enabling remembering saves the current choices; later menu changes save them.
Opting out preserves this session's choices and records `remember=false`, so the
next launch starts with Vanilla. **Reset all to Vanilla** uses the same selection
path as individual rows and saves once when remembering is enabled.

Selection delegates to `lns_palette_select(character, index)`. Portrait and world
sprites therefore use the same selected index, including a refresh while paused,
without advancing fractional portrait or animator phase. Debug selection calls
remain session-only; persistence belongs to explicit palette-menu actions.

Six [preset rows](menu-presets.md), added 2026-11-09, follow Reset and set
every character at once through the same selection and saving path. The status
row is now the 45th focusable row.

## Storage contract

MMAPI owns the location:
`<CONFIG_DIRECTORY>/mod_data/lns_palette/lns_palette.json`. The version-1 envelope
contains `__config_version: 1`, an exact boolean `remember`, and a `selections`
object whose keys are stable character IDs and whose values are stable preset IDs.
Display labels and array positions are never persisted. Normal game settings and
save files are untouched by palette actions.

Loading runs once, either in the registered installer or lazily when the native
Settings screen opens on the title screen. Native Setup has initialized registries,
Anchor and file IO before interactive title input; this does not drain other mods'
installer queues or change MMAPI lifecycle flags. It does
not materialize a default configuration file. MMAPI's path helper can lazily
create the mod directory. Missing configuration leaves Vanilla/off. Invalid
roots, versions, required fields or known-character value types leave
Vanilla/off with a visible message and diagnostic. Unknown characters are ignored.
When remembering is on, missing or unavailable preset IDs fall back to Vanilla
for those characters with a diagnostic. Valid IDs survive a change in preset
ordering.

The pinned MMAPI loader can recover a valid `.bak` when an existing primary does
not parse; the page reports recovery. A missing primary does not resurrect a
stale backup. A parseable but unsupported schema does not recover from backup.
No recovery path writes at startup.

MMAPI's writer swallows failures and provides no success result. The controls
therefore read the **primary file directly**, validate its schema, and compare its
boolean and every known character's stable preset ID with the requested state.
A matching recovery backup does not establish successful primary storage. This
is readback confirmation, not an atomic-write guarantee.

A failed or unconfirmed write leaves current choices usable and immediately
posts a native InfoToasts notification with **Not saved**, without moving the
menu's focus or scroll. Failed opt-out explicitly warns that previous saved
choices may return next launch. Load warnings and recovery messages appear as a
notification when the Palettes page opens. The final status row remains focusable
for rereading; selecting it replays the current diagnostic and never writes.
Successful confirmation clears the message. A completed write can still be
reported unconfirmed if its readback fails; the current session remains usable
in that case too.

## Source-backed integration

`mod/toggle/gml/palette_controls.gml` contains the page and storage behavior; the
existing core hook delegates `Menu.Settings` events to it. Its per-menu category
marker tolerates duplicate delivery. The page callback is bound with `method`
and each row receives explicit menu/character arguments, matching the shipped
native callback invocation semantics rather than closing over a loop variable.

The category uses a unique `lns_palette` page key and an explicit existing gameplay
icon. Its initial native localizer lookup follows the pinned missing-key fallback
contract and its label is immediately replaced with a wrapped raw `Palettes` key.
All later text updates use `.set_key(ANCHOR.wrap_for_local(...))`, preserving raw
labels across the native language-refresh path. The category scroller subscribes
to its pilot so an appended eighth category can scroll into view. Plain element
callbacks avoid native checkbox and option-popup helpers that unconditionally
call `save_settings()`.

Native `InfoToastsMenu.create_notification` accepts a wrapped local key, reflows
its text to the available width and derives toast height from the reflowed text.
The implementation uses that supported surface for immediate feedback instead of
assuming the final scroller row is visible. The persistent status row has 99
pixels of height and participates in the native pilot. The notification boundary
is a delivery spy in interpreter tests; actual wrapped toast rendering passes on title and during gameplay; see the live record.

Source/probe hashes, the pinned dependency checkout and the disposable initial
probe are retained locally under `local/authoring/finish-ui/`. The implementation
record there is `controls-implementation.json`. These local files are evidence,
not redistributable game assets.

## Verification and remaining acceptance

A regression script first failed because the runtime had no `controls` state.
The new opt-in interpreter tests run both production GML modules with all 36
registry identities. Native `create_category`, `element`, callback and text-key
helper bodies and pinned MMAPI config helper bodies are retained in the test
fixture. Drawing, localization, real scrolling/focus and file primitives remain
explicit test boundaries; JSON parsing itself runs in the interpreter.

Run:

```sh
nix-shell --pure --run 'FOM_GML_INTERPRETER="$PWD/local/tools/fabricator/bin/interpreter" cargo test --locked --test palette_controls -- --ignored'
```

The single runtime reliability review found that a trailing noninteractive status
row hid storage warnings from directional navigation. The regression failed before
the fix and passed afterward: failed actions now immediately notify through native
InfoToasts, page opening repeats retained diagnostics, and the focusable status row
can replay a warning without writing. The original focus/scroller is preserved.
No additional review pass was used.

Both focused cases pass. They exercise category idempotence and reopen; all 36
independent rows; explicit/native callback binding; wrapped labels; unchanged
pilot/scroller/focus identity during updates; portrait/world refresh with retained
fractional phase; release hotkey absence; and zero native-settings writes. Storage
checks cover no configuration-file startup writes, stable-ID restart after preset
reordering, opt-in/opt-out/reset, invalid roots/schema/types, unknown characters,
missing/unavailable presets, truncated primary/backup recovery, throwing reads,
throwing/no-op/truncated/wrong-readback writes, session usability, and visible
failure. The review follow-up first reproduced missing immediate notification
from a failed top-row write, then passed after adding native notifications. It
also verifies page-open notifications for invalid/recovered configuration,
failed-opt-out notification text, unchanged focus/scroll during failed writes,
and focusable status replay with zero writes. A separate case retains a matching
valid backup while primary write fails, proving backup recovery cannot mask
failure. Combined repository checks
and the task reliability gate are owned by the integration task.

At the original implementation checkpoint, the remaining live matrix in a
disposable game configuration/save directory included native Settings
from title and journal, keyboard/controller navigation to the eighth category and
through 36 rows, mouse/wheel input, long-label wrapping, back/cancel/close,
language refresh, dialogue and room/save transitions, rendered portrait/world
animation. Actual `CONFIG_DIRECTORY` writes, restart and permission-failure checks passed separately in the disposable engine probe.
Real user saves/settings must remain unexposed during these checks. A generic
Fabricator buffer/file stub does not establish engine IO behavior. GPU availability
and reservation requirements apply to live rendering work.

## Live follow-up, 2026-11-06

Hardware rendering is established. The first actual title/world runs exposed two
integration gaps hidden by the earlier API fixtures: the title room does not drain
MMAPI's gameplay installer queue, and the four cameo characters do not have
regular `NpcId` objects. The current bounded corrections and fresh verification
belong in the [live release record](live-release-verification-2026-11-06.md).
The fresh-title interpreter regression failed before lazy initialization and all
three controls cases pass after it. Fresh title/journal IO/input, all 36 rows, both native toast lifetimes, remembering/
reset/restart and failed opt-out now pass through actual OS/SDL input in the final
installed game. The four-cameo lookup and title initialization corrections are
integrated. The user gave qualified acceptance of the displayed live review;
physical-controller acceptance and broader gameplay limits remain recorded in
the live release record. The earlier API fixtures remain historical evidence.
