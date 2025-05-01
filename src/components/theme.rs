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
    pub dark_text: String,
}

impl Theme {
    pub fn selected_style(&self, selected: bool) -> String {
        let bg = if selected { &self.accent } else { &self.secondary };
        let hover = if selected { &self.accent_hover } else { &self.secondary_hover };
        format!("{} {} {}", bg, hover, &self.dark_text).to_string()
    }
    pub fn button(&self) -> String {
        format!("{} {} cursor-pointer rounded-xl p-2 hover:shadow-lg/80 hover:scale-[1.03] {}", &self.primary, &self.primary_hover, &self.dark_text)
    }
}

// DYNAMIC CSS KEYS
// grid-cols-1
// grid-cols-2
// grid-cols-3
// grid-cols-4
// grid-cols-5
// grid-cols-6
// col-span-1
// col-span-2
// col-span-3
// col-span-4
// col-span-5
// col-span-6
// lg:grid-cols-1
// lg:grid-cols-2
// lg:grid-cols-3
// lg:grid-cols-4
// lg:grid-cols-5
// lg:grid-cols-6
// lg:col-span-1
// lg:col-span-2
// lg:col-span-3
// lg:col-span-4
// lg:col-span-5
// lg:col-span-6
// dark
