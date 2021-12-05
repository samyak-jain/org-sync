use orgize::{indextree::NodeId, Org};

pub struct Task {
    uuid: String,
    title: String,
    description: String,
    keyword: Option<String>,
    priority: u8,
    parent: String,
}

fn parse_section(ast: &Org, section_node: NodeId) -> String {
    section_node
        .children(ast.arena())
        .map(|child| parse_generic(ast, child))
        .collect::<Vec<_>>()
        .join("")
}

fn parse_link(link: &orgize::elements::Link) -> String {
    todo!()
}

fn parse_list(ast: &Org, node: NodeId) -> String {
    node.children(ast.arena())
        .filter_map(|child| {
            if let orgize::Element::ListItem(item) = ast.arena()[child].get() {
                Some(format!(
                    "{}{}{}",
                    "".repeat(item.indent),
                    item.bullet,
                    parse_section(ast, child)
                ))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_timestamp(timestamp: &orgize::elements::Timestamp) -> String {
    "".to_string()
}

fn parse_generic(ast: &Org, node: NodeId) -> String {
    let node_element = ast.arena()[node].get();

    match node_element {
        orgize::Element::SourceBlock(block) => block.contents.to_string(),
        orgize::Element::Section => parse_section(ast, node),
        orgize::Element::Link(link) => format!(
            "[{}]({})",
            link.desc.as_ref().map_or("".to_string(), |d| d.to_string()),
            link.path
        ),
        orgize::Element::List(_) => parse_list(ast, node),
        orgize::Element::Text { value } => value.to_string(),
        orgize::Element::Paragraph { .. } => format!("\n{}\n", parse_section(ast, node)),
        orgize::Element::Timestamp(timestamp) => parse_timestamp(timestamp),
        orgize::Element::Bold => "**".to_string(),
        orgize::Element::Strike => "~~".to_string(),
        orgize::Element::Italic => "*".to_string(),
        orgize::Element::Verbatim { value } => value.to_string(),
        orgize::Element::Code { value } => format!("\n```\n{}\n```\n", value),
        orgize::Element::Table(table) => match table {
            orgize::elements::Table::Org { tblfm, .. } => {
                tblfm.as_ref().map_or("".to_string(), |t| t.to_string())
            }
            orgize::elements::Table::TableEl { value, .. } => value.to_string(),
        },
        _ => "".to_string(),
    }
    .trim()
    .to_string()
}

fn get_parent_uuid(ast: &Org, node: NodeId) -> String {
    todo!()
}

pub fn org_to_task(ast: &Org) -> Vec<Task> {
    let result = ast.headlines().collect::<Vec<_>>()[1];
    println!("result: {:#?}", result);

    let headline_node = result.headline_node();
    let title_node = result.title_node();
    let section_node = result.section_node();

    let arena = ast.arena();
    let headline = &arena[headline_node];
    let title = &arena[title_node];
    if let Some(sec) = section_node {
        println!("headline: {:#?}, title: {:#?}", headline, title);
        println!("section: {:#?}", parse_generic(&ast, sec));
    }

    ast.headlines().map(|headline| {
        let title = headline.title(&ast).clone();

        let description = if let Some(section_node) = headline.section_node() {
            parse_generic(ast, section_node)
        } else {
            "".to_string()
        };

        Task {
            uuid: title
                .properties
                .into_hash_map()
                .get("UUID")
                .expect("Task does not contain UUID property")
                .to_string(),
            title: title.raw.to_string(),
            description,
            keyword: title.keyword.map(|keyword| keyword.to_string()),
            priority: title
                .priority
                .map(|priority| priority as u8 - 64)
                .unwrap_or(0),
            parent: todo!(),
        }
    });

    vec![]
}
