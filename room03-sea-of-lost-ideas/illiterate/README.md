# Illiterate

## Quick Start

```bash
# Install (only build from source for the moment)
cargo install --path .

# To run on your documentation
illiterate --source ./docs check   # Validate references
illiterate --source ./docs tangle  # Generate files
illiterate --source ./docs watch   # Auto-regenerate on changes

# With custom config
illiterate --config examples/entangled.toml --source examples/entangled tangle
```

## How It Works

Write your code inside your documentation. Each code block can:

1. **Be referenced** by other blocks
2. **Emit code** to a file
3. **Include other blocks** via references

````markdown
<!-- inside a `main.md` file -->
# Generate main.rs

```{.txt name=greeting}
Hello World!
```

```{.rust name=hello-fn}
fn greet() -> &'static str {
    "<<greeting>>"
}
```

```{.rust file=src/main.rs}
fn main() {
    println!("{}", <<hello-fn>>);
}
```
````

Run `illiterate --source . tangle` and get `src/main.rs` with all references resolved.

## Code Block Syntax

| Block Type | Syntax |
|------------|--------|
| Named | `{.lang name=name}` |
| File | `{.lang file=path.rs}` |

Named blocks can be included in other blocks using `<<name>>` references.

**Notes:**
- Duplicate named blocks are joined with newlines
- Default regex ignore code blocks inside HTML comments (`<!-- ... -->`)

### Entangled Block Syntax (`#name`)

The `#name` entangled syntax (e.g., `{.rust #myname}`) requires configuring the regex and name key:

```toml
# illiterate.toml
regex_meta_params = '(?<key>#(?!=)|[[:alnum:]]+(?==))=?(?<value>[^\s]+)'
params_name_key = "#"
```

Run with: `cargo run -- --config examples/entangled.toml --source examples/entangled`

## Error Handling

Illiterate detects and reports several error conditions (but it will still emit all the code that is resolvable):

| Code | Meaning |
|------|---------|
| 0 | Success |
| 1 | Circular dependencies detected |
| 2 | Missing references detected |

## Configuration

Configure via CLI flags, environment variables, or `illiterate.toml`:

```toml
# illiterate.toml

# Log level
log_level = "info"

# File extension to search for
filetype = "md"

# Parameter keys for block metadata
params_name_key = "name"   # Use name=value to specify name of the code block
params_file_key = "file"   # Use file=path to specify output

# Custom regex patterns to support formats others than markdown
# You can customize the regex to support your syntax/documentation language
# Just beware of catastrophic backtracking https://www.regular-expressions.info/catastrophic.html
# You can debug them for free here https://regex101.com/
regex_code_block = '(?m)(?:(?<=<!--\O*-->\O*)|(?<!<!--\O*))(?<backquotes>^````*+)(?<meta>.*)\n(?<content>\O*?)\n?\k<backquotes>'
regex_code_meta = '{\.(?<lang>[^\s}]+) ?(?<params>[^}]*)}'
regex_meta_params = '(?<key>[[:alnum:]]+)=(?<value>[^\s]+)'
regex_code_refs = '(?m)^(?:(?![\t ]+<<)(?<line_indent>[\t ]*+)(?:[^\n<]*+|<)+|(?<direct_indent>[\t ]*))(?<full_ref><<(?<ref>[^>]+)>>)'
```

### Configuration Precedence

CLI flags > Environment variables > Config file > Defaults

## Examples

The `examples/` directory contains examples (that are used by the integrations tests):

| Example | Description |
|---------|-------------|
| `basic` | Simple named blocks |
| `file-output` | Emits multiple files to disk |
| `nested-refs` | Deep reference chains |
| `duplicate-blocks` | Duplicate named blocks are joined |
| `comment-blocks` | Code blocks in HTML comments are ignored |
| `circular-deps` | Circular dependency detection |
| `missing-refs` | Missing reference detection |
| `stable-order` | Duplicate named blocks in differents files are always joined in the same order |
| `entangled` | Basic example from entangled website to use with the config `examples/entangled.toml` |
