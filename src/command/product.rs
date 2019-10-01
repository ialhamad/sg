use std::fs;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

const PRODUCTS_PATH: &str = "/home/ihamad/source/github/ui-workspace/ui-products";

#[derive(StructOpt, Debug)]
pub struct Product {
  #[structopt(short)]
  create_product: bool,
  #[structopt(parse(from_os_str))]
  product_name: PathBuf,
}

impl Product {
  pub fn run(self) {
    let Product {
      product_name,
      create_product,
    } = self;

    let path = Path::new(&PRODUCTS_PATH).join(&product_name);
    if create_product {
      fs::create_dir(&path).expect("err");
    }
  }
}
