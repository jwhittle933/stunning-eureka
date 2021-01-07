mod engine {
    use super::text::Text;
    use super::Args;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::{fs, io};

    #[derive(Debug)]
    pub struct Engine<'a> {
        pub src: &'a Text,
        pub chapter: i32,
        pub store: HashMap<&'static str, &'a PathBuf>,
    }

    impl<'a> Engine<'a> {
        pub fn new(a: &Args) -> Engine {
            let engine = Engine {
                src: &a.src,
                chapter: a.chapter,
                store: read_available_files(a.src.location().to_owned())
                    .ok()
                    .expect("problem"),
            };

            engine
        }
    }

    fn read_available_files(loc: String) -> io::Result<HashMap<&'static str, &'static PathBuf>> {
        let contents = fs::read_dir(loc)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<PathBuf>, io::Error>>()?;

        let mut m: HashMap<&str, &PathBuf> = HashMap::new();

        contents.iter().map(|p| {
            let fname = p
                .to_path_buf()
                .as_path()
                .file_name()
                .expect("could not resolve file name");
            m.insert(fname.to_str().expect("invalid operation"), p);
        });

        Ok(m)
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
    use std::io;

    const EXIT: &'static str = "exit";
    const QUIT: &'static str = "quit";

    #[derive(Debug)]
    pub struct Controller {
        pub line: String,
    }

    impl Controller {
        pub fn new() -> Controller {
            Controller {
                line: String::new(),
            }
        }
        pub fn read(&self) -> io::Result<(bool, String)> {
            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            match line.trim() {
                EXIT => Ok((false, "".to_owned())),
                QUIT => Ok((false, "".to_owned())),
                l => Ok((true, String::from(l))),
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
use controller::Controller;
use engine::Engine;

fn main() -> std::io::Result<()> {
    let args = Args::new();
    let engine = Engine::new(&args);
    let controller = Controller::new();

    println!("{}", args.book);
    println!("{:?}", args.action);
    println!("{:?}", engine);
    println!("{:?}", controller);

    loop {
        let (cont, line) = controller.read()?;
        if !cont {
            break;
        }

        println!("{}", line);
    }

    Ok(())
}
