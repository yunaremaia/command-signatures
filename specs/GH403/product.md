# Product specification: `du --time` completion

## Issue

[GH403: du completion spec missing the `--time` long option](https://github.com/warpdotdev/command-signatures/issues/403)

## Problem

Warp's `du` completion spec does not offer GNU `du`'s `--time` option. Users therefore receive no completion for showing timestamps alongside disk-usage output and no guidance for the optional `--time=WORD` values.

The current spec is otherwise usable for its existing BSD-style short options, including the commonly combined `-h` option. This change should add the requested GNU option without changing those completions.

## Desired behavior

- Offer `--time` as a top-level `du` option.
- Describe bare `--time` as showing the last modification time of files or directories in the output.
- Keep the option's argument optional so both `du --time` and `du --time=WORD` are represented by one completion entry.
- When completing the optional `WORD`, offer every value documented by GNU `du`: `atime`, `access`, `use`, `ctime`, and `status`.
- Explain that `atime`, `access`, and `use` select last access time, while `ctime` and `status` select last status-change time.
- Preserve every existing `du` option and file/directory argument completion unchanged.

## User-visible details

`du --time` remains useful without an argument and uses modification time in that form. After the equals sign, completion candidates should make the two synonym groups understandable instead of presenting five unexplained words.

The completion must not imply that the optional value can only be supplied or that the default modification-time behavior requires a value. It must also avoid suggesting undocumented values such as `mtime`.

## Acceptance criteria

1. Typing a `du` option prefix can surface `--time` with a description of its modification-time behavior.
2. Accepting or entering bare `--time` does not require another token.
3. Completing `du --time=` offers exactly `atime`, `access`, `use`, `ctime`, and `status`.
4. Each value communicates whether it selects access time or status-change time.
5. The five values insert their exact GNU-supported spelling.
6. The existing options and the optional variadic file/folder argument in `command-signatures/json/du.json` remain unchanged.
7. The updated JSON is formatted, deserializes through the repository's completion schema, and passes the repository presubmit checks.

## Out of scope

- Adding `--time-style=STYLE`.
- Adding other missing GNU long options, including `--max-depth`.
- Pairing existing short options with GNU long aliases.
- Changing option-name conventions across the repository.
- Adding or changing Rust generators, schemas, or completion-engine behavior.

## Open questions

1. Should synonymous values be displayed as two grouped alias suggestions or five independent suggestions? **Recommendation:** use grouped aliases—`atime`/`access`/`use` and `ctime`/`status`—with one description per semantic group. This matches the existing `ls --color` pattern, produces all five candidates, and avoids duplicated descriptions.
2. Should neighboring `--time-style=STYLE` be included? **Recommendation:** keep it out of scope. It has separate behavior and values, and the issue only requests `--time`; it can be added in a follow-up issue.
