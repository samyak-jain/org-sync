use std::{error::Error, fs};

use clap::Parser;
use log::{debug, trace};
use orgize::Org;
use rusqlite::{
    params,
    types::{ToSqlOutput, ValueRef},
    Connection, ToSql,
};
use stable_eyre::eyre::Report;

mod database;
mod org;
mod tasks;

// fn convert_to_org(org_json: String) -> Result<String, Report> {
//     let new_org: Org = serde_json::from_str(&org_json).unwrap();
//
//     let mut res_vec = Vec::new();
//     new_org.write_org(&mut res_vec)?;
//
//     let new_org_str = String::from_utf8(res_vec)?;
//     Ok(new_org_str)
// }

fn store() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("./store.db").unwrap();

    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS org (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            file TEXT NOT NULL,
            data TEXT NOT NULL
            );
        ",
        [],
    )?;

    let org_file_content = fs::read_to_string("./data/keywords.org")?;
    let parsed_org = Org::parse(&org_file_content);

    let org_json = serde_json::to_string(&parsed_org)?;

    let sql_json = org_json.to_sql()?;
    if let ToSqlOutput::Borrowed(ValueRef::Text(text)) = sql_json {
        conn.execute(
            "INSERT INTO org (file, data) VALUES (?1, ?2)",
            params!["./data/keywords.org", text],
        )?;
    }

    Ok(())
}

fn parse_org() -> Result<(), Box<dyn Error>> {
    let org_file_content = fs::read_to_string("./data/keywords.org")?;
    let parsed_org = Org::parse(&org_file_content);

    for node in parsed_org.arena().iter() {
        println!("{:#?}", node);
    }

    Ok(())
}

#[derive(Parser)]
#[clap(version = "0.1-alpha")]
struct Opts {
    #[clap(short, long, default_value = ".")]
    directory: String,

    #[clap(long, arg_enum, default_value = "error")]
    log: LogLevel,

    #[clap(long, default_value = "./store.db")]
    database: String,

    #[clap(subcommand)]
    commands: Commands,
}

#[derive(clap::ArgEnum, Clone)]
enum LogLevel {
    Debug,
    Error,
    Trace,
}

impl Into<log::LevelFilter> for LogLevel {
    fn into(self) -> log::LevelFilter {
        match self {
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Trace => log::LevelFilter::Trace,
        }
    }
}

#[derive(Parser)]
enum Commands {
    ParseOrg,
    ParseIcal,
}

fn setup_logger(level: log::LevelFilter) -> Result<(), fern::InitError> {
    let colors = fern::colors::ColoredLevelConfig::new();

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn main() {
    stable_eyre::install().unwrap();
    let opts = Opts::parse();

    setup_logger(opts.log.into()).expect("Unable to setup logger");

    debug!("Test if logging is working");

    let mut connection =
        database::setup_database(&opts.database).expect("Could not setup database");

    match opts.commands {
        Commands::ParseOrg => {
            let org_file_paths = org::read_org_directory(&opts.directory);

            for org_file_path in org_file_paths {
                debug!("Org File Path: {}", org_file_path.path().to_string_lossy());

                let file_name = org_file_path
                    .path()
                    .to_str()
                    .expect("File path is not valid UTF-8");

                let file_contents = fs::read_to_string(org_file_path.path())
                    .expect(&format!("Could not read file: {}", file_name));

                debug!("Org File Contents: \n{}", file_contents);

                let org_ast = org::text_to_ast(&file_contents);

                // trace!("Org arena: {:#?}", org_ast.arena());

                tasks::org_to_task(&org_ast);

                // database::first_time_database_entry(&org_ast, &connection, file_name)
                //     .expect("Could not handle first time entry into the database");

                // database::update_database(&org_ast, &mut connection, file_name)
                //     .expect("Could not update database");
            }
        }
        Commands::ParseIcal => todo!(),
    }
}
