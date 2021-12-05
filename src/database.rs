use chrono::Utc;
use fallible_streaming_iterator::FallibleStreamingIterator;
use log::{debug, trace};
use orgize::{indextree::NodeEdge, Element, Org};
use rusqlite::{params, Connection};
use stable_eyre::eyre::Report;

use crate::tasks::Task;

pub fn setup_database(database_path: &str) -> Result<Connection, Report> {
    let conn = Connection::open(database_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            uuid TEXT NOT NULL PRIMARY KEY, 
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            keyword TEXT,
            PRIORITY INTEGER NOT NULL,
            parent TEXT,
            updated_timestamp DATETIME NOT NULL,
            reason TEXT CHECK( reason IN ('ical', 'org') ) NOT NULL);",
        [],
    )?;

    // conn.execute(
    //     "CREATE UNIQUE INDEX IF NOT EXISTS org_file_index ON org (file);",
    //     [],
    // )?;

    Ok(conn)
}

pub fn insert_tasks(conn: &Connection, tasks: Vec<Task>) {}

// pub fn update_database<'a>(
//     ast: &Org<'a>,
//     conn: &mut Connection,
//     file_name: &'a str,
// ) -> Result<(), Report> {
//     let transaction = conn.transaction()?;
// 
//     let mut fetch_node_statement =
//         transaction.prepare("SELECT json_extract(data, ?1) FROM org WHERE file = ?2")?;
// 
//     for node in ast.root.traverse(ast.arena()) {
//         if let NodeEdge::Start(node_id) = node {
//             let node_item = &ast.arena()[node_id].get();
// 
//             let mut node_index: usize = node_id.into();
//             node_index -= 1;
// 
//             trace!("Item: {:#?}", node_item);
//             trace!("Index: {}", node_index);
// 
//             if let Some(column) = fetch_node_statement
//                 .query_map::<String, _, _>(
//                     params![
//                         format!("$.arena.nodes[{}].data.Data", node_index),
//                         file_name
//                     ],
//                     |row| row.get(0),
//                 )?
//                 .next()
//                 .transpose()?
//             {
//                 trace!("column: {}", column);
// 
//                 let element: Element = serde_json::from_str(&column)?;
// 
//                 debug!("Element: {:#?}", element);
// 
//                 if element == **node_item {
//                     trace!("elements are equal at Index: {}", node_index);
//                     continue;
//                 }
//             }
// 
//             let node_json = serde_json::to_string(node_item)?;
// 
//             debug!("Node JSON: {}", node_json);
// 
//             transaction.execute(
//                 "UPDATE org SET data = (SELECT json_set(data, ?1, json(?2)) \
//                     FROM org WHERE file = ?3) WHERE file = ?3",
//                 params![
//                     format!("$.arena.nodes[{}]", node_index),
//                     node_json,
//                     file_name
//                 ],
//             )?;
// 
//             transaction.execute(
//                 "INSERT OR REPLACE INTO updated (id, node, file, time, reason) \
//                     VALUES ((SELECT id FROM updated WHERE file = ?1 AND node = ?2),\
//                         ?2, ?1, ?3, 'org')",
//                 params![file_name, node_index, Utc::now()],
//             )?;
//         }
//     }
// 
//     drop(fetch_node_statement);
//     transaction.commit()?;
// 
//     Ok(())
// }

enum UpdateReason {
    ICAL,
    ORG,
}

// TODO: Change timestamp to use chrono
fn update_timestamp(conn: Connection, timestamp: String, reason: UpdateReason) {}
