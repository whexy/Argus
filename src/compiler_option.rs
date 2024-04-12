#[derive(Debug, Clone)]
pub struct CompilerOption {
    pub name: String,
    pub values: Vec<String>,
    pub is_enabled: bool,
    pub has_value: bool,
}

impl CompilerOption {
    pub fn new(name: &str) -> Self {
        CompilerOption {
            name: name.to_string(),
            values: Vec::new(),
            is_enabled: true,
            has_value: false,
        }
    }

    pub fn new_with_value(name: &str, value: Vec<&str>) -> Self {
        CompilerOption {
            name: name.to_string(),
            values: value.iter().map(|&v| v.to_string()).collect(),
            is_enabled: true,
            has_value: true,
        }
    }

    pub fn from_arg(arg: &str) -> Self {
        let parts: Vec<&str> = arg.splitn(2, '=').collect();
        let name = parts[0].to_string();
        let values = match parts.get(1) {
            Some(v) => v.split(',').map(|v| v.to_string()).collect(),
            None => Vec::new(),
        };
        let has_value = !values.is_empty();

        CompilerOption {
            name,
            values,
            is_enabled: true,
            has_value,
        }
    }

    pub fn contains(&self, value: &str) -> bool {
        self.values.iter().any(|v| v == value)
    }

    /// Check if the new_value is already present in the value list. If not, add it to the list.
    pub fn add_or_update_value(&mut self, new_value: &str) {
        if !self.has_value {
            panic!("Option {} cannot have a value", self.name)
        }
        if !self.contains(new_value) {
            self.values.push(new_value.to_string());
        }
    }

    /// Remove the value from the list. After that, if the list is empty, disable the option.
    pub fn remove_value(&mut self, value_to_remove: &str) {
        if !self.has_value {
            panic!("Option {} cannot have a value", self.name)
        }
        self.values.retain(|v| v != value_to_remove);
        if self.values.is_empty() {
            self.disable();
        }
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn enable(&mut self) {
        self.is_enabled = true;
    }
}

pub trait OptionManagement {
    fn get_option(&self, name: &str) -> Option<&CompilerOption>;
    fn get_mut_option(&mut self, name: &str) -> Option<&mut CompilerOption>;
    fn add_or_modify(&mut self, option: &CompilerOption);
    fn add_or_mix(&mut self, option: &CompilerOption);
}

impl OptionManagement for Vec<CompilerOption> {
    fn get_option(&self, name: &str) -> Option<&CompilerOption> {
        self.iter().find(|opt| opt.name == name && opt.is_enabled)
    }

    fn get_mut_option(&mut self, name: &str) -> Option<&mut CompilerOption> {
        self.iter_mut()
            .find(|opt| opt.name == name && opt.is_enabled)
    }

    /// Add the option if it does not exist, otherwise modify the value.
    /// If the option does not exist, add it to the list.
    /// If the option exists, its value will be replaced with the new value.
    fn add_or_modify(&mut self, other: &CompilerOption) {
        if let Some(opt) = self.get_mut_option(&other.name) {
            if opt.has_value {
                opt.values = other.values.clone();
            }
        } else {
            self.push(other.clone());
        }
    }

    /// Add the option if it does not exist, otherwise update the value.
    /// If the option does not exist, add it to the list.
    /// If the option exists, and it does not have a value, set the value to the new value.
    /// If the option exists, and has a value, try mixing the new value with the existing value.
    fn add_or_mix(&mut self, other: &CompilerOption) {
        if let Some(opt) = self.get_mut_option(&other.name) {
            for value in &other.values {
                opt.add_or_update_value(value);
            }
        } else {
            self.push(other.clone());
        }
    }
}

impl std::fmt::Display for CompilerOption {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if !self.is_enabled {
            return Ok(());
        }
        if self.has_value {
            write!(f, "{}={}", self.name, self.values.join(","))
        } else {
            write!(f, "{}", self.name)
        }
    }
}
