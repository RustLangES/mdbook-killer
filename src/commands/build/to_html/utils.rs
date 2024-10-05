use std::fmt::Write;

pub struct ToHtmlBuffer {
    pub buffer: String,
    beautify: bool,
    indent: u8,

    pub widgets: dashmap::DashMap<String, usize>,
}

impl ToHtmlBuffer {
    pub fn new(beautify: bool) -> Self {
        Self {
            buffer: String::new(),
            beautify,
            indent: 0,
            widgets: dashmap::DashMap::new(),
        }
    }

    pub fn push_newline(&mut self) {
        if self.beautify {
            self.buffer.push('\n');
            self.buffer.push_str(&"  ".repeat(self.indent as usize));
        }
    }

    pub fn push_indent(&mut self) {
        if self.beautify {
            self.indent += 1;
        }
    }

    pub fn pop_indent(&mut self) {
        if self.beautify {
            self.indent -= 1;
        }
    }

    pub fn tag<'a>(&'a mut self, tag: &'a str, attrs: &str) -> Tag<'a> {
        Tag::open(self, tag, attrs)
    }
}

impl Write for ToHtmlBuffer {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.buffer.write_str(s)
    }
}

pub struct Tag<'a> {
    pub buffer: &'a mut ToHtmlBuffer,
    tag: &'a str,
}

impl Tag<'_> {
    pub fn open<'a>(buffer: &'a mut ToHtmlBuffer, tag: &'a str, attrs: &str) -> Tag<'a> {
        _ = buffer.write_char('<');
        _ = buffer.write_str(tag);
        _ = buffer.write_char(' ');
        _ = buffer.write_str(attrs);
        _ = buffer.write_char('>');

        buffer.push_indent();
        buffer.push_newline();

        Tag { buffer, tag }
    }

    pub fn close(self) {
        drop(self)
    }
}

impl Drop for Tag<'_> {
    fn drop(&mut self) {
        self.buffer.pop_indent();
        self.buffer.push_newline();

        _ = self.buffer.write_char('<');
        _ = self.buffer.write_char('/');
        _ = self.buffer.write_str(self.tag);
        _ = self.buffer.write_char('>');
    }
}
