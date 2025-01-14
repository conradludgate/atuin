use std::env;

use eyre::Result;
use structopt::StructOpt;

use crate::local::database::{Database, Sqlite};
use crate::local::history::History;

#[derive(StructOpt)]
pub enum Cmd {
    #[structopt(
        about="begins a new command in the history",
        aliases=&["s", "st", "sta", "star"],
    )]
    Start { command: Vec<String> },

    #[structopt(
        about="finishes a new command in the history (adds time, exit code)",
        aliases=&["e", "en"],
    )]
    End {
        id: String,
        #[structopt(long, short)]
        exit: i64,
    },

    #[structopt(
        about="list all items in history",
        aliases=&["l", "li", "lis"],
    )]
    List {
        #[structopt(long)]
        distinct: bool,
    },
}

impl Cmd {
    pub fn run(&self, db: &mut Sqlite) -> Result<()> {
        match self {
            Self::Start { command: words } => {
                let command = words.join(" ");
                let cwd = env::current_dir()?.display().to_string();

                let h = History::new(
                    chrono::Utc::now().timestamp_nanos(),
                    command,
                    cwd,
                    -1,
                    -1,
                    None,
                    None,
                );

                // print the ID
                // we use this as the key for calling end
                println!("{}", h.id);
                db.save(h)?;
                Ok(())
            }

            Self::End { id, exit } => {
                let mut h = db.load(id)?;
                h.exit = *exit;
                h.duration = chrono::Utc::now().timestamp_nanos() - h.timestamp;

                db.update(h)?;

                Ok(())
            }

            Self::List { distinct } => db.list(*distinct),
        }
    }
}
