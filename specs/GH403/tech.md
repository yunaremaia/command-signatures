# Technical specification: `du --time` completion

## Current implementation

`command-signatures/json/du.json` is a single Fig-compatible JSON command definition with:

- Fourteen option entries: `-a`, `-c`, `-H`, `-h`, `-g`, `-k`, `-m`, `-I`, `-L`, `-r`, `-P`, `-d`, `-s`, and `-x`.
- No option whose name begins with `--`.
- One optional, variadic top-level `files` argument using the `filepaths` and `folders` templates.

The JSON is embedded and deserialized through `command-signatures/src/lib.rs`. Its `all_command_specs_succeed_deserialization` test loads every embedded signature through the completion metadata schema.

## Recommended implementation

Add one object to the existing `options` array in `command-signatures/json/du.json`:

```json
{
    "name": "--time",
    "description": "Show the time of the last modification of any file in the directory, or any of its subdirectories",
    "args": {
        "name": "WORD",
        "isOptional": true,
        "suggestions": [
            {
                "name": [
                    "atime",
                    "access",
                    "use"
                ],
                "description": "Show the last access time"
            },
            {
                "name": [
                    "ctime",
                    "status"
                ],
                "description": "Show the last status-change time"
            }
        ]
    }
}
```

This is one option with one optional argument, not separate entries for bare `--time` and each `--time=WORD` spelling. It follows `command-signatures/json/ls.json`'s `--color` structure: `isOptional` preserves the bare form, while a suggestion `name` array expands aliases into separate insertable candidates with a shared description. `command-signatures/json/eza.json`'s `--icons` option independently confirms the optional fixed-suggestion shape.

Use uppercase `WORD` because GNU `du --help` documents the syntax as `--time[=WORD]`, consistent with the repository authoring guide's instruction to match the command's help-text casing. Do not add `requiresEquals`; the reference `ls --color` option omits it, and no completion-engine or schema change is part of this issue.

## Files and components affected

- `command-signatures/json/du.json`: add the `--time` option and static value suggestions.

No Rust source, generator registration, schema, dependency, or API change is required. The spec documents under `specs/GH403/` are workflow artifacts and remain on this branch for the implementation stage.

## Data and completion behavior

The two suggestion objects intentionally encode semantic aliases:

- `["atime", "access", "use"]` becomes three candidates that share the access-time description.
- `["ctime", "status"]` becomes two candidates that share the status-change-time description.

The conversion in `completion-metadata/src/fig_types.rs` expands each `name` array into individual internal suggestions and copies the shared description to each one. Static suggestions are sufficient; no shell process or `generatorName` is needed.

The optional argument must not change the behavior of the command's existing optional variadic `files` argument. Bare `--time` must remain complete by itself, while an equals-sign value context must expose the five fixed candidates.

## Edge cases

- Do not suggest `mtime`: GNU `du` uses modification time for bare `--time`, but does not document `mtime` as a `WORD` value.
- Do not collapse the five accepted spellings into only the two conceptual time types; all documented aliases must remain insertable.
- Do not add `--time-style` while editing the neighboring option surface.
- Do not pair or rewrite existing short options as part of this change; issues #372 and #374 track related but separate work.
- Preserve the current file/folder templates and option exclusivity metadata.

## Validation and testing

The implementation must:

1. Format the edited JSON:

   `npm run format -- command-signatures/json/du.json`

2. Run the repository presubmit from the repository root:

   `script/presubmit`

   This verifies all JSON formatting with `npm run format:check`, runs `cargo fmt --check`, runs Clippy with warnings denied, and runs the full Rust test suite. The test suite includes deserializing every embedded command spec, which validates the new optional argument and suggestion aliases against the actual schema.

3. Inspect the resulting completion behavior, preferably in a local Warp session:
   - `du --t` can offer `--time`.
   - `du --time=` offers all five documented values.
   - Selecting each candidate inserts its exact spelling.
   - Bare `du --time` does not force an argument.

No generator screenshot is required because this change adds only static suggestions and no generator. CI in `.github/workflows/CI.yml` separately repeats JSON formatting, Cargo formatting and Clippy, and the full Rust test suite.

## Open questions

The implementation depends on maintainer confirmation of the product decisions in `product.md`: grouped synonym suggestions are recommended, and `--time-style=STYLE` is recommended to remain out of scope.
