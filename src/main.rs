use clap::{load_yaml, App};
use sg::command::SetProduct;
use std::env;
fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    if let Some(matches) = matches.subcommand_matches("product") {
        if let Some(c) = matches.value_of("product_name") {
            SetProduct::new(c.to_owned()).run();
        }
        if let Some(c) = matches.value_of("create_product") {
            println!("test {}", c);
        }
    }
    println!("{}", env::var("VOLTA_HOME").unwrap());
}
