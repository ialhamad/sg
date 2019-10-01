pub use self::add::Add;
pub use self::product::Product;
pub mod add;
pub mod product;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
  name = "Lime",
  global_setting(structopt::clap::AppSettings::ColoredHelp),
  global_setting(structopt::clap::AppSettings::ColorAlways),
  global_setting(structopt::clap::AppSettings::DeriveDisplayOrder),
  global_setting(structopt::clap::AppSettings::DisableVersion),
  global_setting(structopt::clap::AppSettings::DontCollapseArgsInUsage),
  global_setting(structopt::clap::AppSettings::VersionlessSubcommands)
)]
pub struct Lime {
  #[structopt(subcommand)]
  pub command: Option<Subcommand>,

  #[structopt(
    short = "v",
    long = "version",
    help = "Prints the current version of Lime"
  )]
  pub version: bool,
}

impl Lime {
  pub fn run(self) {
    if let Some(command) = self.command {
      command.run();
    }
  }
}

#[derive(StructOpt, Debug)]
pub enum Subcommand {
  Product(Product),
  Add(Add),
}

impl Subcommand {
  pub fn run(self) {
    match self {
      Subcommand::Product(product) => product.run(),
      Subcommand::Add(add) => add.run(),
    }
  }
}
