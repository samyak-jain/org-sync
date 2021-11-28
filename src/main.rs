use std::{error::Error, fs};

use clap::Parser;
use orgize::Org;
use rusqlite::{
    params,
    types::{ToSqlOutput, ValueRef},
    Connection, ToSql,
};
use stable_eyre::eyre::Report;
use walkdir::DirEntry;

mod database;
mod org;

fn convert_to_org(org_json: String) -> Result<String, Report> {
    let new_org: Org = serde_json::from_str(&org_json).unwrap();

    let mut res_vec = Vec::new();
    new_org.write_org(&mut res_vec)?;

    let new_org_str = String::from_utf8(res_vec)?;
    Ok(new_org_str)
}

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

    #[clap(short, long, default_value = "./store.db")]
    database: String,

    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Parser)]
enum Commands {
    ParseOrg,
    ParseiCal,
}

fn main() {
    let opts = Opts::parse();

    let mut connection =
        database::setup_database(&opts.database).expect("Could not setup database");

    match opts.commands {
        Commands::ParseOrg => {
            let org_file_paths = org::read_org_directory(&opts.directory)
                .collect::<Result<Vec<DirEntry>, walkdir::Error>>()
                .expect("Could not read from directory");

            for org_file_path in org_file_paths {
                let file_name = org_file_path
                    .file_name()
                    .to_str()
                    .expect("File name is not valid UTF-8");

                let file_contents = fs::read_to_string(org_file_path.path())
                    .expect(&format!("Could not read file: {}", file_name));

                let org_ast = org::text_to_ast(&file_contents);

                database::first_time_database_entry(&org_ast, &connection, file_name)
                    .expect("Could not handle first time entry into the database");

                database::update_database(&org_ast, &mut connection, file_name)
                    .expect("Could not update database");
            }
        }
        Commands::ParseiCal => todo!(),
    }
}
