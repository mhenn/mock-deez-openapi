mod yaml;
use yaml::get_path_keys;

use crate::yaml::get_yaml;

fn main() {
    let yaml = get_yaml("./src/test.yaml").expect("Failed opening yaml");
    let keys = get_path_keys(&yaml);
    println!("{:?}", keys)
}
