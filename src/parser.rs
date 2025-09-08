use pulldown_cmark::{Event, Tag, TagEnd, TextMergeStream};
use std::fmt;
use std::fs;
use std::path;

const TODO_PREFIX: &'static str = "TODO:";

pub enum TagType {
    TODO,
}

pub enum TagItemContent {
    List {
        is_numbered: bool,
        items: Vec<String>,
    },
}

impl fmt::Display for TagType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TagType::TODO => write!(f, "TODO"),
        }
    }
}

pub struct TagItem {
    pub tag_type: TagType,
    pub title: String,
    pub content: Option<TagItemContent>,
}

pub fn parse_file(file_path: String) -> std::io::Result<FileInfo> {
    let file_content = fs::read_to_string(&file_path)?;
    let parser: pulldown_cmark::Parser = pulldown_cmark::Parser::new(&file_content);
    let mut iter = TextMergeStream::new(parser);

    let mut file_info = new_file_info(file_path);
    file_info.handle_generic_events(&mut iter);

    Ok(file_info)
}

pub struct FileInfo {
    pub file_path: path::PathBuf,
    pub todos: Vec<TagItem>,
}

fn new_file_info(file_path: String) -> FileInfo {
    let mut path_buf = path::PathBuf::new();
    path_buf.push(file_path);
    return FileInfo {
        file_path: path_buf,
        todos: Vec::new(),
    };
}

impl FileInfo {
    // Generic "stateless" event handler (parser starts here as the initial state)
    fn handle_generic_events<'a>(
        &mut self,
        iter: &mut TextMergeStream<'a, pulldown_cmark::Parser<'a>>,
    ) {
        let event = match iter.next() {
            Some(event) => event,
            None => return,
        };
        match event {
            Event::Start(tag) => match tag {
                Tag::Heading { .. } => self.handle_heading(iter, &mut String::new()),
                _ => self.handle_generic_events(iter),
            },
            _ => self.handle_generic_events(iter),
        }
    }

    // Handler for within heading state
    fn handle_heading<'a>(
        &mut self,
        iter: &mut TextMergeStream<'a, pulldown_cmark::Parser<'a>>,
        buf: &mut String,
    ) {
        let event = match iter.next() {
            Some(event) => event,
            None => return,
        };
        match event {
            Event::Text(txt) => {
                let text = &txt.into_string();
                buf.push_str(text);
                self.handle_heading(iter, buf);
            }
            Event::End(tag_end) => match tag_end {
                TagEnd::Heading(_) => {
                    if let Some(title) = buf.strip_prefix(TODO_PREFIX) {
                        // After an item prefix, expect item content
                        self.handle_tag_item_content(iter, TagType::TODO, title);
                    }
                    self.handle_generic_events(iter);
                }
                _ => self.handle_heading(iter, buf),
            },
            _ => self.handle_heading(iter, buf),
        }
        return;
    }

    fn handle_tag_item_content<'a>(
        &mut self,
        iter: &mut TextMergeStream<'a, pulldown_cmark::Parser<'a>>,
        tag_type: TagType,
        title: &str,
    ) {
        let event = match iter.next() {
            Some(ev) => ev,
            None => {
                self.todos.push(TagItem {
                    tag_type: tag_type,
                    title: title.to_string(),
                    content: None,
                });
                // End tag item
                return self.handle_generic_events(iter);
            }
        };
        match event {
            Event::Start(tag) => match tag {
                Tag::List(num) => {
                    let list = self.collect_list(iter, num != None);
                    self.todos.push(TagItem {
                        tag_type: tag_type,
                        title: title.to_string(),
                        content: Some(list),
                    });
                    return self.handle_generic_events(iter);
                }
                _ => {}
            },
            _ => {}
        };
        self.todos.push(TagItem {
            tag_type: tag_type,
            title: title.to_string(),
            content: None,
        });
        // End tag item
        self.handle_generic_events(iter);
    }

    fn collect_list<'a>(
        &mut self,
        iter: &mut TextMergeStream<'a, pulldown_cmark::Parser<'a>>,
        is_numbered: bool,
    ) -> TagItemContent {
        let mut items: Vec<String> = Vec::new();
        while let Some(event) = iter.next() {
            match event {
                Event::End(tag_end) => {
                    if let TagEnd::List(_) = tag_end {
                        break;
                    }
                },
                Event::Start(tag) => {
                    if tag == Tag::Item {
                        let l_item_content = self.collect_list_item(iter);
                        items.push(l_item_content);
                    }
                },
                _ => {
                    break;
                }
            }
        }
        return TagItemContent::List {
            is_numbered: is_numbered,
            items: items,
        };
    }

    fn collect_list_item<'a>(
        &mut self,
        iter: &mut TextMergeStream<'a, pulldown_cmark::Parser<'a>>,
    ) -> String {
        let mut str_buf = String::new();
        while let Some(event) = iter.next() {
            if let Event::Text(content) = event {
                str_buf.push_str(&content);
                continue;
            }
            break;
        }
        return str_buf;
    }
}
