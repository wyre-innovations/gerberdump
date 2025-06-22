# gerberdump

`gerberdump` is a command-line tool for analyzing Gerber files (PCB fabrication files) to extract detailed information about PCB designs, manufacturing requirements, and cost factors. It's designed for hobbyists, professionals, and manufacturers who need to understand Gerber files without visual rendering.

## Installation

Build from source:
```bash
git clone https://github.com/wyre-innovations/gerberdump
cd gerberdump
cargo build --release
```

## Usage

### Basic Usage

```bash
# Default mode: Fabrication cost analysis
gerberdump /path/to/gerber/files

# Analyze current directory
gerberdump .

# Gerber X2 analysis mode
gerberdump --x2 /path/to/gerber/files
```

### Analysis Modes

| Command | Description |
|---------|-------------|
| `--fab-cost` | Analyze fabrication cost factors and manufacturing complexity |
| `-2, --x2` | Gerber X2 format analysis (attributes and metadata) |
| `-H, --file-headers` | Display file headers and metadata (X2 attributes) |
| `-a, --apertures` | Display aperture definitions and their properties |
| `-c, --commands` | Display all commands in the file |
| `-g, --graphics` | Display graphics operations (D01/D02/D03) and coordinates |
| `-r, --regions` | Display region definitions (G36/G37) |
| `-b, --blocks` | Display block definitions (AB) |
| `-s, --step-repeat` | Display step and repeat definitions (SR) |
| `-x, --attributes` | Display all attributes (X2 format) |
| `-m, --macros` | Display macro definitions (AM) |
| `-f, --format` | Display coordinate format and units |
| `--graphics-state` | Display graphics state changes |
| `-t, --transformations` | Display aperture transformations (LP, LM, LR, LS) |
| `--validate` | Validate file format and report errors |
| `--stats` | Display statistics about the file |

### Output Formats

| Format | Description |
|--------|-------------|
| `--output-format human` | Human-readable format (default) |
| `--output-format json` | JSON format |
| `--output-format xml` | XML format |
| `--output-format csv` | CSV format (for tabular data) |
| `--output-format raw` | Raw format (minimal processing) |

### Filtering Options

| Command | Description |
|---------|-------------|
| `--aperture <NUM>` | Filter by aperture number (can be used multiple times) |
| `--file-function <FUNCTION>` | Filter by file function (for X2 files) |
| `--layer <LAYER>` | Filter by layer (Top, Bottom, Inner, etc.) |
| `--deprecated` | Show only deprecated commands/features |

### Directory Processing

| Command | Description |
|---------|-------------|
| `-R, --recursive` | Recurse into subdirectories |
| `--include-hidden` | Include hidden files when processing directories |
| `--pattern <PATTERN>` | File pattern to match (e.g., "*.gbr", "*.ger") |

### Display Options

| Command | Description |
|---------|-------------|
| `-v, --verbose` | Verbose output (show additional details) |
| `-q, --quiet` | Quiet mode (suppress non-essential output) |
| `-n, --line-numbers` | Display line numbers with output |
| `--offsets` | Show file offsets for commands |
| `--raw` | Display raw command strings |
| `--limit <N>` | Maximum number of items to display (0 = unlimited) |

## Fabrication Cost Analysis

The default mode analyzes key factors that affect PCB manufacturing cost and complexity:

- **Board dimensions** - Affects material cost and panelization
- **Layer count** - Affects complexity and manufacturing cost
- **Via count and types** - Through-hole, blind, buried vias
- **Drill hole sizes and counts** - Affects tooling and setup costs
- **Minimum trace width and spacing** - Affects yield and capability requirements
- **Copper coverage percentage** - Affects material usage
- **Number of unique apertures** - Affects setup time and tooling
- **Special features** - Slots, cutouts, castellations
- **Surface finish requirements** - From X2 attributes
- **Solder mask and silkscreen complexity** - Additional layers
- **Manufacturing difficulty score** - Overall complexity rating

## Examples

```bash
# Analyze fabrication cost factors (default)
gerberdump ./gerber_files

# Full X2 analysis with verbose output
gerberdump --x2 --verbose ./gerber_files

# Show only aperture information in JSON format
gerberdump --apertures --output-format json ./gerber_files

# Validate files and show deprecated features
gerberdump --validate --deprecated ./gerber_files

# Analyze specific apertures only
gerberdump --aperture 10 --aperture 11 --aperture 12 ./gerber_files

# Recursive analysis with file pattern
gerberdump --recursive --pattern "*.gbr" /path/to/pcb/projects

# Show graphics operations with line numbers
gerberdump --graphics --line-numbers --verbose ./gerber_files

# Manufacturing analysis with CSV output for spreadsheet
gerberdump --fab-cost --output-format csv ./gerber_files > cost_analysis.csv
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - see [LICENSE](LICENSE) file for details.
