

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NewsLanguage {
    Code(&'static str),
    Default,
}

impl NewsLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Code(c) => c,
            Self::Default => "",
        }
    }

    // Now takes the specific client's list as an argument
    pub fn from_str(s: &str, supported_list: &[(&'static str, &'static str)]) -> Self {
        if s.is_empty() { return Self::Default; }
        
        // Find the code in the client's provided list
        if let Some((static_code, _)) = supported_list.iter().find(|(code, _)| *code == s) {
            Self::Code(static_code)
        } else {
            Self::Default
        }
    }

    // Now takes the specific client's list as an argument
    pub fn display_name(&self, supported_list: &[(&'static str, &'static str)]) -> &'static str {
        match self {
            Self::Default => "All Languages",
            Self::Code(c) => supported_list.iter()
                .find(|(code, _)| code == c)
                .map(|(_, name)| *name)
                .unwrap_or("Unknown"),
        }
    }
}