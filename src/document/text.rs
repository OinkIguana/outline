/// A [`TextBlock`] is just text that will be copied verbatim into the output documentation file
#[derive(Debug, Default)]
pub struct TextBlock<'a> {
    /// The source text
    text: Vec<&'a str>
}

impl<'a> TextBlock<'a> {
    /// Creates a new empty [`TextBlock`]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_line(mut self, line: &'a str) -> Self {
        self.text.push(line);
        self
    }

    pub fn to_string(&self) -> String {
        self.text.join("\n")
    }
}
