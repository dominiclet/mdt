use pulldown_cmark::{Event, Tag, TagEnd, TextMergeStream};
use std::fmt;
use std::fs;
use std::path;

const TODO_PREFIX: &'static str = "TODO:";

pub enum TagType {
    TODO,
}

impl fmt::Display for TagType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TagType::TODO => write!(f, "TODO"),
        }
    }
}

pub struct FileInfo {
    pub file_path: path::PathBuf,
    pub todos: Vec<TagItem>,
}

pub struct TagItem {
    pub tag_type: TagType,
    pub content: String,
}

pub fn parse_file(file_path: String) -> std::io::Result<FileInfo> {
    let file_content = fs::read_to_string(&file_path)?;
    let parser: pulldown_cmark::Parser = pulldown_cmark::Parser::new(&file_content);
    let iter = TextMergeStream::new(parser);

    let mut within_heading: bool = false;
    let mut heading_buffer: String = String::new();
    let mut todos: Vec<TagItem> = Vec::new();

    for event in iter {
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { .. } => {
                    within_heading = true;
                }
                _ => {}
            },
            Event::End(tag_end) => match tag_end {
                TagEnd::Heading(_) => {
                    within_heading = false;
                    if let Some(todo_content) = heading_buffer.strip_prefix(TODO_PREFIX) {
                        todos.push(TagItem {
                            tag_type: TagType::TODO,
                            content: todo_content.trim().to_string(),
                        });
                    }
                    heading_buffer.clear();
                }
                _ => {}
            },
            Event::Text(txt) => {
                if within_heading {
                    let text = &txt.into_string();
                    heading_buffer.push_str(text);
                }
            }
            _ => {}
        }
    }
    let mut path_buf = path::PathBuf::new();
    path_buf.push(file_path);

    Ok(FileInfo {
        file_path: path_buf,
        todos: todos,
    })
}
