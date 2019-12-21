// Main code

#[derive(Clap)]
pub struct Head {
    #[clap(parse(from_os_str))]
    pub file: PathBuf,
    #[clap(short = "n", default_value = "5")]
    pub count: usize,
}

// Build code

use clap::IntoApp;
use clap_generate::gen_manuals;

#[path="src/cli.rs"]
mod cli;

fn main() {
    let app = cli::Head::into_app();
    for man in gen_manuals(&app) {
        let name = "head.1";
        let mut out = fs::File::create("head.1").unwrap();

        use std::io::Write;
        out.write_all(man.render().as_bytes()).unwrap();
    }
}
