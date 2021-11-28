use chrono::Utc;
use fallible_streaming_iterator::FallibleStreamingIterator;
use orgize::{indextree::NodeEdge, Element, Org};
use rusqlite::{params, Connection};
use stable_eyre::eyre::Report;

pub fn setup_database(database_path: &str) -> Result<Connection, Report> {
    let conn = Connection::open(database_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS org (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
            file TEXT NOT NULL,
            data TEXT NOT NULL);",
        [],
    )?;

    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS org_file_index ON org (file);",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS updated (
            id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
            node INTEGER NOT NULL, 
            file TEXT NOT NULL, 
            time DATETIME NOT NULL,
            reason TEXT CHECK( reason IN ('ical', 'org') ) NOT NULL);",
        [],
    )?;

    conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS updated_file_node ON updated (file, node)",
        [],
    )?;

    Ok(conn)
}

pub fn first_time_database_entry(
    ast: &Org,
    conn: &Connection,
    file_name: &str,
) -> Result<(), Report> {
    let mut fetch_entry_statement = conn.prepare("SELECT * FROM org WHERE file = ?1")?;

    if fetch_entry_statement.exists(params![file_name])? {
        return Ok(());
    }

    let json_string = serde_json::to_string(&ast)?;

    conn.execute(
        "INSERT INTO org (file, data) VALUES (?1, json(?2))",
        params![file_name, json_string],
    )?;

    Ok(())
}

pub fn update_database<'a>(
    ast: &Org<'a>,
    conn: &mut Connection,
    file_name: &'a str,
) -> Result<(), Report> {
    let transaction = conn.transaction()?;

    let mut fetch_node_statement = transaction
        .prepare("SELECT json_extract(data, '$.arena.nodes[?1]') FROM org WHERE file = ?2")?;

    for node in ast.root.traverse(ast.arena()) {
        if let NodeEdge::Start(node_id) = node {
            let node_item = &ast.arena()[node_id].get();
            let node_index: usize = node_id.into();

            if let Some(row) = fetch_node_statement
                .query(params![node_index, file_name])?
                .get()
            {
                let column: String = row.get(0)?;
                let element: Element = serde_json::from_str(&column)?;

                println!("Element: {:#?}", element);

                // TODO: Fix this error
                // if PartialEq::eq(&element, &node_item) {
                //     continue;
                // }
            }

            let node_json = serde_json::to_string(node_item)?;
            transaction.execute(
                    "UPDATE org SET data = (SELECT json_set(data, '$.arena.nodes[?1]', json(?2)) FROM org WHERE file = ?3) WHERE file = ?3"
                    , params![node_index, node_json, file_name]
            )?;

            transaction.execute(
                    "INSERT OR REPLACE INTO updated (id, node, file, time, reason) VALUES ((SELECT id FROM updated WHERE file = ?1 AND node = ?2), ?2, ?1, ?3, 'org')",
                    params![file_name, node_index, Utc::now()],
            )?;
        }
    }

    drop(fetch_node_statement);
    transaction.commit()?;

    Ok(())
}

enum UpdateReason {
    ICAL,
    ORG,
}

// TODO: Change timestamp to use chrono
fn update_timestamp(conn: Connection, timestamp: String, reason: UpdateReason) {}
