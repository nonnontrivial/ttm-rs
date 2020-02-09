mod app;
mod input;

pub(crate) use self::input::{Digraph, Source};
use anyhow::Result;
use structopt::StructOpt;

fn main() -> Result<()> {
    let args = app::Opt::from_args();
    let source = Source::from(args.files);
    let digraph = Digraph::new(args.tuple_separator)?;
    digraph.build(&source, args.overwrite)?;
    Ok(())
}
