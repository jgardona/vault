use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    user: Option<String>,
    password: Option<String>,
    description: Option<String>,
}

impl Item {
    pub fn new(user: Option<&str>, password: Option<&str>, description: Option<&str>) -> Self {
        Self {
            user: user.map(|s| s.to_owned()),
            password: password.map(|s| s.to_owned()),
            description: description.map(|s| s.to_owned()),
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let user = self.user.as_deref().unwrap_or_default();
        let password = self.password.as_deref().unwrap_or_default();
        let description = self.description.as_deref().unwrap_or_default();

        write!(f, "{user:>30}{password:>30}{description:>30}")
    }
}

#[cfg(test)]
mod model_tests {
    use super::Item;

    #[test]
    fn test_item_workflow() {
        let item = Item::new(Some("jcbritobr@gmail.com"), Some("123"), Some("A password"));
        // test display conversion
        let expected = format!(
            "{:>30}{:>30}{:>30}",
            "jcbritobr@gmail.com", "123", "A password"
        );
        let result = format!("{item}");
        assert_eq!(expected, result);
    }
}
