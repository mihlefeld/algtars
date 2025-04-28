#[derive(Clone, PartialEq)]
pub struct Theme {
    pub dark: bool,
    pub text: String,
    pub background: String,
    pub primary: String,
    pub primary_hover: String,
    pub secondary: String,
    pub secondary_hover: String
}

impl Theme {
    pub fn selected_style(&self, selected: bool) -> String {
        let bg = if selected { &self.primary } else { &self.secondary };
        let hover = if selected { &self.primary_hover } else { &self.secondary_hover };
        format!("{} {}", bg, hover).to_string()
    }
}