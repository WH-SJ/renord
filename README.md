# renord

A CLI tool for batch renaming files. Supports reordering by a name list or find-and-replace with optional regex.

## Install

```bash
cargo install --path .
```

## Usage

```bash
renord <DIR> [OPTIONS] <COMMAND>
```

### Global Options

| Option | Description |
|--------|-------------|
| `--dry-run` | Preview changes without actually renaming |
| `--ignore-case` | Case-insensitive matching (replace only) |

---

### `reorder` — Rename by name list

Reads new names from a file and renames matched files in order, resulting in `index.name.ext`.

```bash
renord ./videos reorder --ext mp4 --names-file names.txt --sort-mod name
```

**Options**

| Option | Description |
|--------|-------------|
| `-e, --ext <EXT>` | File extension to match, e.g. `mp4` |
| `-n, --names-file <FILE>` | Text file with one new name per line |
| `-s, --sort-mod <MOD>` | Sort order: `name` / `size` / `modified` / `created` |

**Example**

`names.txt`:
```
intro
chapter-one
chapter-two
```

Result:
```
ep01.mp4 → 1.intro.mp4
ep02.mp4 → 2.chapter-one.mp4
ep03.mp4 → 3.chapter-two.mp4
```

---

### `replace` — Find and replace in filenames

Matches a string in filenames and replaces it, with optional regex support.

```bash
renord ./videos replace --from "episode_" --to "ep."
```

**Options**

| Option | Description |
|--------|-------------|
| `-f, --from <STR>` | String or regex pattern to match |
| `-t, --to <STR>` | Replacement string |
| `-e, --ext <EXT>` | Only process files with this extension (default: all) |
| `--regex` | Treat `--from` as a regular expression |

**Examples**

```bash
# Plain replace
renord ./videos replace --from "episode_" --to "ep."
# episode_01.mp4 → ep.01.mp4

# Regex replace
renord ./videos replace --from "ep\d+" --to "episode" --regex
# ep01.mp4 → episode.mp4

# Case-insensitive
renord ./videos replace --from "EPISODE" --to "ep." --ignore-case
# episode_01.mp4 → ep.01.mp4

# Dry run preview
renord ./videos replace --from "old" --to "new" --dry-run
```

## License

MIT
