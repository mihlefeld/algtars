#[derive(Clone, PartialEq)]
pub struct Theme {
    pub dark: bool,
    pub text: String,
    pub background: String,
    pub accent_background: String,
    pub primary: String,
    pub primary_hover: String,
    pub primary_text: String,
    pub primary_text_hover: String,
    pub secondary: String,
    pub secondary_hover: String,
    pub accent: String,
    pub accent_hover: String,
    pub dark_text: String,
}

impl Theme {
    pub fn selected_style(&self, selected: bool) -> String {
        let bg = if selected { &self.accent } else { &self.secondary };
        let hover = if selected { &self.accent_hover } else { &self.secondary_hover };
        format!("{} {} {}", bg, hover, &self.dark_text).to_string()
    }
    pub fn button(&self) -> String {
        format!("{} {} cursor-pointer rounded-xl p-2 hover:shadow-lg/80 hover:scale-[1.03] shadow-md/80 {}", &self.primary, &self.primary_hover, &self.dark_text)
    }
    pub fn text_link(&self) -> String {
        format!("{} {} cursor-pointer underline", &self.primary_text, &self.primary_text_hover)
    }
}

// DYNAMIC CSS KEYS
// dark