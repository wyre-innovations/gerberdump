use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "gerberdump")]
#[command(about = "objdump for Gerber files - analyze Gerber Layer Format files")]
#[command(version = "0.1.0")]
#[command(author = "Wyre Innovations")]
#[command(long_about = None)]
struct Cli {
    /// Input file or directory containing Gerber files
    #[arg(value_name = "FILE|DIR")]
    input: PathBuf,

    /// Display file headers and metadata (X2 attributes)
    #[arg(short = 'H', long = "file-headers")]
    file_headers: bool,

    /// Display aperture definitions and their properties
    #[arg(short = 'a', long = "apertures")]
    apertures: bool,

    /// Display all commands in the file
    #[arg(short = 'c', long = "commands")]
    commands: bool,

    /// Display graphics operations (D01/D02/D03) and coordinates
    #[arg(short = 'g', long = "graphics")]
    graphics: bool,

    /// Display region definitions (G36/G37)
    #[arg(short = 'r', long = "regions")]
    regions: bool,

    /// Display block definitions (AB)
    #[arg(short = 'b', long = "blocks")]
    blocks: bool,

    /// Display step and repeat definitions (SR)
    #[arg(short = 's', long = "step-repeat")]
    step_repeat: bool,

    /// Display all attributes (X2 format)
    #[arg(short = 'x', long = "attributes")]
    attributes: bool,

    /// Gerber X2 format analysis (default mode, includes attributes and metadata)
    #[arg(short = '2', long = "x2")]
    x2_mode: bool,

    /// Display macro definitions (AM)
    #[arg(short = 'm', long = "macros")]
    macros: bool,

    /// Display coordinate format and units
    #[arg(short = 'f', long = "format")]
    format_info: bool,

    /// Display graphics state changes
    #[arg(long = "graphics-state")]
    graphics_state: bool,

    /// Display aperture transformations (LP, LM, LR, LS)
    #[arg(short = 't', long = "transformations")]
    transformations: bool,

    /// Validate file format and report errors
    #[arg(long = "validate")]
    validate: bool,

    /// Display statistics about the file
    #[arg(long = "stats")]
    statistics: bool,

    /// Analyze fabrication cost factors and manufacturing complexity
    #[arg(long = "fab-cost", alias = "manufacturing")]
    fabrication_cost: bool,

    /// Output format
    #[arg(short = 'o', long = "output-format", value_enum, default_value_t = OutputFormat::Human)]
    output_format: OutputFormat,

    /// Filter by aperture number (can be used multiple times)
    #[arg(long = "aperture", value_name = "NUM")]
    filter_aperture: Vec<i32>,

    /// Filter by file function (for X2 files)
    #[arg(long = "file-function", value_name = "FUNCTION")]
    filter_file_function: Option<String>,

    /// Filter by layer (Top, Bottom, Inner, etc.)
    #[arg(long = "layer", value_name = "LAYER")]
    filter_layer: Option<String>,

    /// Show only deprecated commands/features
    #[arg(long = "deprecated")]
    show_deprecated: bool,

    /// Verbose output (show additional details)
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    /// Quiet mode (suppress non-essential output)
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,

    /// Recurse into subdirectories when input is a directory
    #[arg(short = 'R', long = "recursive")]
    recursive: bool,

    /// Include hidden files when processing directories
    #[arg(long = "include-hidden")]
    include_hidden: bool,

    /// File pattern to match (e.g., "*.gbr", "*.ger")
    #[arg(long = "pattern", value_name = "PATTERN")]
    file_pattern: Option<String>,

    /// Display line numbers with output
    #[arg(short = 'n', long = "line-numbers")]
    line_numbers: bool,

    /// Show file offsets for commands
    #[arg(long = "offsets")]
    show_offsets: bool,

    /// Display raw command strings
    #[arg(long = "raw")]
    raw_commands: bool,

    /// Maximum number of items to display (0 = unlimited)
    #[arg(long = "limit", value_name = "N", default_value_t = 0)]
    limit: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum OutputFormat {
    /// Human-readable format (default)
    Human,
    /// JSON format
    Json,
    /// XML format  
    Xml,
    /// CSV format (for tabular data)
    Csv,
    /// Raw format (minimal processing)
    Raw,
}

impl Cli {
    /// Returns true if any analysis mode is explicitly selected
    fn has_explicit_mode(&self) -> bool {
        self.file_headers
            || self.apertures
            || self.commands
            || self.graphics
            || self.regions
            || self.blocks
            || self.step_repeat
            || self.attributes
            || self.macros
            || self.format_info
            || self.graphics_state
            || self.transformations
            || self.validate
            || self.statistics
            || self.fabrication_cost
    }

    /// Returns true if X2 mode should be used (either explicitly or as default)
    fn should_use_x2_mode(&self) -> bool {
        self.x2_mode || (!self.has_explicit_mode() && !self.should_use_fab_cost_mode())
    }

    /// Returns true if fabrication cost mode should be used as default
    fn should_use_fab_cost_mode(&self) -> bool {
        !self.has_explicit_mode() && !self.x2_mode
    }
}

fn main() {
    let cli = Cli::parse();

    // Determine which default mode to use
    if cli.should_use_fab_cost_mode() && !cli.quiet {
        println!("Using Fabrication Cost Analysis mode (default)");
        println!("Input: {}", cli.input.display());
        if cli.verbose {
            println!("Fab cost mode analyzes: board dimensions, layer count, via density,");
            println!("drill sizes, trace widths, copper coverage, and manufacturing complexity");
        }
    } else if cli.should_use_x2_mode() && !cli.quiet {
        println!("Using Gerber X2 analysis mode");
        println!("Input: {}", cli.input.display());
        if cli.verbose {
            println!("X2 mode includes: file headers, attributes, apertures, and format info");
        }
    }

    // Display what would be analyzed based on selected options
    if !cli.quiet {
        let mut analysis_modes = Vec::new();

        if cli.file_headers || cli.should_use_x2_mode() {
            analysis_modes.push("file headers");
        }
        if cli.apertures || cli.should_use_x2_mode() {
            analysis_modes.push("apertures");
        }
        if cli.commands {
            analysis_modes.push("commands");
        }
        if cli.graphics {
            analysis_modes.push("graphics operations");
        }
        if cli.regions {
            analysis_modes.push("regions");
        }
        if cli.blocks {
            analysis_modes.push("blocks");
        }
        if cli.step_repeat {
            analysis_modes.push("step & repeat");
        }
        if cli.attributes || cli.should_use_x2_mode() {
            analysis_modes.push("attributes");
        }
        if cli.should_use_fab_cost_mode() {
            analysis_modes.push("fabrication cost factors");
        }
        if cli.macros {
            analysis_modes.push("macros");
        }
        if cli.format_info || cli.should_use_x2_mode() {
            analysis_modes.push("format info");
        }
        if cli.graphics_state {
            analysis_modes.push("graphics state");
        }
        if cli.transformations {
            analysis_modes.push("transformations");
        }
        if cli.validate {
            analysis_modes.push("validation");
        }
        if cli.statistics {
            analysis_modes.push("statistics");
        }
        if cli.fabrication_cost {
            analysis_modes.push("fabrication cost analysis");
        }

        if !analysis_modes.is_empty() {
            println!("Analysis modes: {}", analysis_modes.join(", "));
        }

        // Display filters if any are set
        if !cli.filter_aperture.is_empty() {
            println!("Filtering by apertures: {:?}", cli.filter_aperture);
        }
        if let Some(ref func) = cli.filter_file_function {
            println!("Filtering by file function: {}", func);
        }
        if let Some(ref layer) = cli.filter_layer {
            println!("Filtering by layer: {}", layer);
        }

        println!("Output format: {:?}", cli.output_format);

        if cli.recursive {
            println!("Recursive directory processing enabled");
        }
        if let Some(ref pattern) = cli.file_pattern {
            println!("File pattern: {}", pattern);
        }
    }

    // Show fabrication cost factors if in fab cost mode
    if cli.should_use_fab_cost_mode() && !cli.quiet {
        println!("\nFabrication Cost Factors to be analyzed:");
        println!("  • Board dimensions (affects material cost)");
        println!("  • Layer count (affects complexity and cost)");
        println!("  • Via count and types (through-hole, blind, buried)");
        println!("  • Drill hole sizes and counts (affects tooling)");
        println!("  • Minimum trace width and spacing (affects yield)");
        println!("  • Copper coverage percentage (affects material)");
        println!("  • Number of unique apertures (affects setup time)");
        println!("  • Special features (slots, cutouts, castellations)");
        println!("  • Surface finish requirements");
        println!("  • Solder mask and silkscreen complexity");
        println!("  • Manufacturing difficulty score");
        println!("");
    }

    // TODO: Implement actual Gerber file parsing and analysis
    println!("Gerber file parsing implementation coming soon...");

    if cli.verbose {
        println!("\nConfiguration:");
        println!("  Input: {}", cli.input.display());
        println!("  Verbose: {}", cli.verbose);
        println!("  Quiet: {}", cli.quiet);
        println!("  Show deprecated: {}", cli.show_deprecated);
        println!("  Line numbers: {}", cli.line_numbers);
        println!("  Show offsets: {}", cli.show_offsets);
        println!("  Raw commands: {}", cli.raw_commands);
        if cli.limit > 0 {
            println!("  Limit: {}", cli.limit);
        }
    }
}
