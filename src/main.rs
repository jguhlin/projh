use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("pjh.yaml");
    let matches = App::from(yaml).get_matches();
}
