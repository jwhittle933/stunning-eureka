mod engine {
    use super::text::Text;
    use super::Args;
    use std::path::PathBuf;
    use std::{fs, io};

    #[derive(Debug)]
    pub struct Engine<'a> {
        pub src: &'a Text,
        pub content: Vec<PathBuf>,
        pub chapter: i32,
    }

    impl<'a> Engine<'a> {
        pub fn new(a: &Args) -> Engine {
            let engine = Engine {
                src: &a.src,
                chapter: a.chapter,
                content: read_available_files(a.src.location().to_owned())
                    .ok()
                    .expect("could not read files"),
            };

            engine
        }
    }

    fn read_available_files(loc: String) -> io::Result<Vec<PathBuf>> {
        let contents = fs::read_dir(loc)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()?;

        Ok(contents)
    }
}

mod text {
    use std::str::FromStr;
    use std::string::ParseError;

    #[derive(Debug)]
    pub enum Text {
        Lxx,
        MT,
    }

    impl Text {
        pub fn location(&self) -> &str {
            match self {
                Text::Lxx => "./src/formatted/lxx/",
                Text::MT => "./src/formatted/mt/",
            }
        }
    }

    impl FromStr for Text {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "lxx" => Ok(Text::Lxx),
                "mt" => Ok(Text::MT),
                _ => Ok(Text::Lxx),
            }
        }
    }
}

mod controller {
    use super::engine::Engine;

    #[derive(Debug)]
    pub struct Controller<'a> {
        engine: &'a Engine<'a>,
    }

    impl <'a> Controller<'a> {
        pub fn new(e: &'a Engine<'a>) -> Controller {
            Controller {
                engine: e,
            }
        }
    }
}

mod args {
    use super::text;
    use structopt::StructOpt;

    #[derive(StructOpt)]
    pub struct Args {
        #[structopt(short, long)]
        debug: bool,

        #[structopt(short = "A", long = "action", default_value = "read")]
        pub action: String,
        #[structopt(short = "B", long = "book", default_value = "Genesis")]
        pub book: String,
        #[structopt(short = "C", long = "chapter", default_value = "1")]
        pub chapter: i32,
        #[structopt(short = "S", long = "src", default_value = "lxx")]
        pub src: text::Text,
    }

    impl Args {
        pub fn new() -> Args {
            Args::from_args()
        }
    }
}

use args::Args;
use engine::Engine;
use controller::Controller;

fn main() -> std::io::Result<()> {
    let args = Args::new();
    let engine = Engine::new(&args);
    let controller = Controller::new(&engine);

    println!("{}", args.book);
    println!("{:?}", args.action);
    println!("{:?}", engine);
    println!("{:?}", controller);

    for c in engine.content {
        println!(
            "{:?}",
            c.as_path()
                .file_name()
                .expect("could not resolve file name")
        );
    }

    Ok(())
}
