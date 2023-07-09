#[derive(PartialEq, Eq, Hash, Clone, Default)]
pub struct NString(String);

impl NString {
    pub fn new(s: impl ToString) -> Self {
        let s = s.to_string();

        let re = regex::Regex::new(r#"\[[0-9a-f]+\]|::\{constructor#\d+\}"#).unwrap();
        let s = re.replace_all(&s, "");
        Self(s.to_string())
    }
}

impl std::fmt::Debug for NString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
        // f.debug_tuple("NString").field(&self.0).finish()
    }
}
