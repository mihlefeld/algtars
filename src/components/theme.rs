#[derive(Clone, PartialEq)]
pub struct Theme {
    pub dark: bool,
    pub text: String,
    pub background: String,
    pub accent_background: String,
    pub primary: String,
    pub primary_hover: String,
    pub secondary: String,
    pub secondary_hover: String,
    pub accent: String,
    pub accent_hover: String,
}

impl Theme {
    pub fn selected_style(&self, selected: bool) -> String {
        let bg = if selected { &self.accent } else { &self.secondary };
        let hover = if selected { &self.accent_hover } else { &self.secondary_hover };
        format!("{} {}", bg, hover).to_string()
    }
}