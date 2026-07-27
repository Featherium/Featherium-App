pub struct Recipe {
    pub id: &'static str,
    pub name: &'static str,
    pub url: &'static str,
    pub icon: &'static str,
    pub accent_color: &'static str,
    pub default_user_agent: &'static str,
}

pub const RECIPES: &[Recipe] = &[Recipe {
    id: "whatsapp",
    name: "WhatsApp",
    url: "https://web.whatsapp.com/",
    icon: "message-circle",
    accent_color: "#25D366",
    default_user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
}];

pub fn find_recipe(id: &str) -> Option<&'static Recipe> {
    RECIPES.iter().find(|recipe| recipe.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_whatsapp_recipe() {
        let recipe = find_recipe("whatsapp").expect("whatsapp recipe must exist");
        assert_eq!(recipe.name, "WhatsApp");
        assert_eq!(recipe.url, "https://web.whatsapp.com/");
    }

    #[test]
    fn unknown_recipe_returns_none() {
        assert!(find_recipe("does-not-exist").is_none());
    }
}
