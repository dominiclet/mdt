use crate::{commands, config, parser};
use colored::Colorize;

const NEWLINE_CHAR: &str = "\n";
const DIVIDER_SEQ: &str = "--------------------------------";
const HARD_DIVIDER_SEQ: &str = "========================================";

pub fn print_status_overview(ctx: &commands::Context, file_infos: Vec<parser::FileInfo>) {
    let mut sb = String::new();
    sb.push_str(get_preamble(&ctx.config).as_str());
    sb.push_str(NEWLINE_CHAR);
    for file_info in file_infos {
        if file_info.todos.len() == 0 {
            continue;
        }
        // Add file name
        let file_name = match file_info.file_path.file_name() {
            Some(file_name) => file_name.to_str().unwrap_or("-"),
            None => "-",
        };
        sb.push_str(file_name);
        sb.push_str(NEWLINE_CHAR);
        sb.push_str(DIVIDER_SEQ);
        sb.push_str(NEWLINE_CHAR);
        // Add tag items
        sb.push_str(format_items(file_info.todos).as_str());
        sb.push_str(NEWLINE_CHAR);
    }
    print!("{}", sb);
}

fn get_preamble(conf: &config::Config) -> String {
    let mut sb = String::new();
    sb.push_str(format!("{}{}", HARD_DIVIDER_SEQ, NEWLINE_CHAR).as_str());
    sb.push_str("mdt\n");
    sb.push_str(format!("Root directory: {}{}", conf.notes_directory, NEWLINE_CHAR).as_str());
    sb.push_str(format!("{}{}", HARD_DIVIDER_SEQ, NEWLINE_CHAR).as_str());
    return sb;
}

fn format_items(items: Vec<parser::TagItem>) -> String {
    let mut res = String::new();
    for item in items {
        let mut item_str = format_item(item);
        item_str.push('\n');
        res.push_str(item_str.as_str());
    }
    res
}

fn format_item(item: parser::TagItem) -> String {
    let tag_part = item.tag_type.to_string() + ":";
    format!("{} {}", tag_part.bright_red(), item.content)
}
