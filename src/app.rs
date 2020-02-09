use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ttm-rs",
    about = "
        CLI tool for turning tuples into adjacency matrices
    "
)]
pub(crate) struct Opt {
    /// Filepaths to tuple-holding files
    #[structopt(short = "f", long = "files")]
    pub files: Vec<String>,
    /// Whether source files should be overwritten
    #[structopt(short = "o", long = "overwrite")]
    pub overwrite: bool,
    /// Character that should separate tuples in source
    #[structopt(short = "d", long = "delimiter")]
    pub tuple_separator: Option<String>,
}
