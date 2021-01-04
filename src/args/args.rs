use structopt::StructOpt;


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
}
