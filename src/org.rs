use orgize::Org;
use walkdir::{DirEntry, WalkDir};

fn is_valid_path(entry: &DirEntry) -> bool {
    let file_name = entry.file_name().to_str();

    let is_hidden = file_name.map(|s| s.starts_with("."));
    let is_valid_org = file_name.map(|s| s.ends_with(".org"));

    is_valid_org
        .zip(is_hidden)
        .map(|(valid_org, hidden)| valid_org && !hidden)
        .unwrap_or(false)
}

pub fn read_org_directory(path: &str) -> impl Iterator<Item = Result<DirEntry, walkdir::Error>> {
    WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| is_valid_path(e))
}

pub fn text_to_ast<'a>(org: &'a str) -> Org<'a> {
    Org::parse(org)
}
