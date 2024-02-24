use clap::{Arg, ArgMatches, Command};
use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use regex::Regex;
use semver::{Version, VersionReq};

use std::io;
use std::process;

fn main() {
    let matches = Command::new("mdbook-dice")
        .about("An mdbook preprocessor for custom dice notation")
        .subcommand(
            Command::new("supports")
                .arg(Arg::new("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .get_matches();

    let preprocessor = DiceNotationPreprocessor;

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    } else if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(mdbook::MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The mdbook-dice was built against version {} of mdbook, \
             but we're being called from version {}",
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.get_one::<String>("renderer").expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

struct DiceNotationPreprocessor;

impl Preprocessor for DiceNotationPreprocessor {
    fn name(&self) -> &str {
        "mdbook-dice"
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let re_normal = Regex::new(r"\[\[((\d+)?[A-Z].*?\d+)\]\]").unwrap();
        let re_advantage = Regex::new(r"\[\[\+((\d+)?[A-Z].*?\d+)\]\]").unwrap();
        let re_disadvantage = Regex::new(r"\[\[\-((\d+)?[A-Z].*?\d+)\]\]").unwrap();

        book.for_each_mut(|item: &mut BookItem| {
            if let BookItem::Chapter(ref mut chapter) = item {
                chapter.content = re_normal.replace_all(&chapter.content, "<span class='dice-roll'>$1</span>").to_string();
                chapter.content = re_advantage.replace_all(&chapter.content, "<span class='dice-roll advantage'>$1</span>").to_string();
                chapter.content = re_disadvantage.replace_all(&chapter.content, "<span class='dice-roll disadvantage'>$1</span>").to_string();
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        // Assuming support for all renderers, but you can customize this logic
        true
    }
}
