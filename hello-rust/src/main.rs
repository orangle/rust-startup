use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};
use structopt::StructOpt;
use log::{info, warn};

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    hello()
}

fn hello() {
    env_logger::init();
    info!("===================start=====================");
    let args = Cli::from_args();
    let resut = std::fs::read_to_string(&args.path);
    let content = match resut {
        Ok(content) => content,
        Err(error) => {
            panic!("can not deal with {}, just exit here", error)
        }
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            //???
            println!("{}", line);
        }
    }

    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;
    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}
