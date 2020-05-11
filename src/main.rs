use bstr::io::BufReadExt;
use lcat::{Rainbow, RainbowCmd};
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};
use structopt::StructOpt;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(StructOpt)]
#[structopt(name = "lcat", about = "Terminal rainbows.")]
pub struct Cmdline {
    #[structopt(name = "File", default_value = "-", parse(from_os_str))]
    files: Vec<PathBuf>,

    #[structopt(flatten)]
    rainbow: RainbowCmd,
}

fn main() -> Result<(), io::Error> {
    let opt = Cmdline::from_args();

    let mut rainbow: Rainbow = opt.rainbow.into();

    for path in opt.files {
        let stdin = io::stdin();
        let stdin = stdin.lock();
        let file: Box<dyn BufRead> = if path == PathBuf::from("-") {
            Box::new(stdin)
        } else {
            let f = File::open(path).unwrap();
            let b = BufReader::new(f);
            Box::new(b)
        };

        let stdout = io::stdout();
        let mut stdout = stdout.lock();
        file.for_byte_line_with_terminator(|ref line| {
            rainbow.colorize(line, &mut stdout)?;
            Ok(true)
        })?;
    }

    Ok(())
}
