use structopt::StructOpt;

mod engine {
    use std::{fs, io};
    use std::path::PathBuf;
    use super::Args;
    use super::text::Text;

    #[derive(Debug)]
    pub struct Engine<'a> {
        pub src: &'a Text,
        pub content: Vec<PathBuf>,
    }

    impl <'a> Engine<'a> {
        pub fn new_from_args(a: &Args) -> Engine {
            let engine = Engine{
                src: &a.src,
                content: read_available_files(a.src.location().to_owned())
                    .ok()
                    .expect("could not read files")
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
                _ => Ok(Text::Lxx)
            }
        }
    }
}

#[derive(StructOpt)]
pub struct Args {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(short = "A", long = "action", default_value = "read")]
    action: String,
    #[structopt(short = "B", long = "book", default_value = "Genesis")]
    book: String,
    #[structopt(short = "C", long = "chapter", default_value = "1")]
    chapter: i32,
    #[structopt(short = "S", long = "src", default_value = "lxx")]
    src: text::Text,
}


fn main() -> std::io::Result<()> {
    let args = Args::from_args();
    let engine = engine::Engine::new_from_args(&args);

    println!("{}", args.book);
    println!("{:?}", args.action);
    println!("{:?}", engine);

    for c in engine.content {
        println!("{:?}", c
            .as_path()
            .file_name()
            .expect("could not resolve file name"));
    }

    Ok(())
}
