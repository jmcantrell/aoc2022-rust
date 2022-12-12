#[derive(Debug, Clone)]
pub enum Command<'a> {
    ListCurrentDirectory,
    ChangeDirectory { name: &'a str },
}
