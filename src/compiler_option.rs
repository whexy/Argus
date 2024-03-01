pub struct CompilerOption {
    pub name: String,
    pub value: Option<String>,
    pub is_enabled: bool,
}

impl CompilerOption {
    pub fn new(name: &str, is_enabled: bool) -> Self {
        let parts: Vec<&str> = name.splitn(2, '=').collect();
        let name = parts[0].to_string();
        let value = parts.get(1).map(|&v| v.to_string());

        CompilerOption {
            name: name,
            value: value,
            is_enabled,
        }
    }

    pub fn add_or_update_value(&mut self, new_value: &str) {
        match self.value {
            Some(ref mut value) => {
                if !value.contains(new_value) {
                    value.push(',');
                    value.push_str(new_value);
                }
            }
            None => {
                self.value = Some(new_value.to_string());
            }
        }
    }

    pub fn remove_value(&mut self, value_to_remove: &str, disable_if_empty: bool) {
        if let Some(value) = &self.value {
            let values: Vec<&str> = value.split(',').filter(|&v| v != value_to_remove).collect();
            if values.is_empty() && disable_if_empty {
                self.is_enabled = false;
                self.value = None;
            } else {
                self.value = Some(values.join(","));
            }
        }
    }
}

impl std::fmt::Display for CompilerOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.is_enabled {
            return Ok(());
        }
        match &self.value {
            Some(value) => write!(f, "{}={}", self.name, value),
            None => write!(f, "{}", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_or_update_value_new_value() {
        let mut option = CompilerOption::new("-fsanitize", true);
        option.add_or_update_value("undefined");
        assert_eq!(option.value, Some("undefined".to_string()));
    }

    #[test]
    fn test_add_or_update_value_update_value() {
        let mut option = CompilerOption::new("-fsanitize=address", true);
        option.add_or_update_value("undefined");
        assert_eq!(option.value, Some("address,undefined".to_string()));
    }

    #[test]
    fn test_add_or_update_value_duplicate_value() {
        let mut option = CompilerOption::new("-fsanitize=address", true);
        option.add_or_update_value("address");
        assert_eq!(option.value, Some("address".to_string()));
    }

    #[test]
    fn test_remove_value_and_disable_if_empty() {
        let mut option = CompilerOption::new("-fsanitize=address,fuzz", true);
        option.remove_value("fuzz", true);

        assert_eq!(option.value, Some("address".to_string()));
        assert!(option.is_enabled);

        option.remove_value("address", true);
        assert!(option.value.is_none());
        assert!(!option.is_enabled);
    }

    #[test]
    fn test_remove_value_without_disabling() {
        let mut option = CompilerOption::new("-fsanitize=fuzz,address", true);
        option.remove_value("fuzz", false);

        assert_eq!(option.value, Some("address".to_string()));
        assert!(option.is_enabled); // 确保选项仍然启用
    }

    #[test]
    fn test_remove_nonexistent_value() {
        let mut option = CompilerOption::new("-fsanitize=address,undefined", true);
        option.remove_value("fuzz", true);

        // 确保值未被修改
        assert_eq!(option.value, Some("address,undefined".to_string()));
        assert!(option.is_enabled);
    }
}
