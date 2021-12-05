use std::fs::OpenOptions;

use eyre::bail;
use log::{debug, trace};
use orgize::{indextree::NodeId, Org};
use stable_eyre::Report;
use walkdir::{DirEntry, WalkDir};

fn is_valid_path(entry: DirEntry) -> Option<DirEntry> {
    trace!("Checking entry {:#?}", entry);

    let file_name = entry.file_name().to_str();
    let is_valid_org = file_name.map(|s| s.ends_with(".org"));

    if is_valid_org == Some(true) {
        Some(entry)
    } else {
        None
    }
}

pub fn read_org_directory(path: &str) -> impl Iterator<Item = DirEntry> {
    debug!("Walking path: {}", path);

    WalkDir::new(path).into_iter().filter_map(|entry| {
        entry
            .ok()
            .and_then(|unwrapped_entry| is_valid_path(unwrapped_entry))
    })
}

pub fn text_to_ast<'a>(org: &'a str) -> Org<'a> {
    Org::parse(org)
}

fn add_uuid_to_task<'a>(org: &mut Org<'a>, task_node: NodeId, uuid: &'a str) -> Result<(), Report> {
    let arena = org.arena_mut();
    let element_node = arena[task_node].get_mut();

    let title = match element_node {
        orgize::Element::Title(title) => title,
        _ => bail!("Incorrect element is being changed"),
    };

    title.properties.pairs.push((
        std::borrow::Cow::Borrowed("UUID"),
        std::borrow::Cow::Borrowed(&uuid),
    ));

    Ok(())
}

fn write_org_changes_to_file(org: &Org, file_name: &str) -> Result<(), Report> {
    let file = OpenOptions::new().write(true).open(file_name)?;
    org.write_org(file);

    Ok(())
}

fn get_parent_uuid(org: &Org, headline_node_id: NodeId) -> String {
    let headline_node = &org.arena()[headline_node_id];

    if let Some(parent_node_id) = headline_node.parent() {
        let parent_node = org.arena()[parent_node_id].get();

        match parent_node {
            orgize::Element::Headline { .. } => {}
            _ => {}
        }
    }

    String::from("")
}
