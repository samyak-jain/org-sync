use log::{debug, trace};
use orgize::Org;
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
