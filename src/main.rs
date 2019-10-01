use sg::command::Lime;
use structopt::StructOpt;

fn main() {
    let lime = Lime::from_args();
    lime.run();
}
