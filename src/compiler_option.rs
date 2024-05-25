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
    fn get_options(&self, name: &str) -> Vec<&CompilerOption>;
    fn get_mut_options(&mut self, name: &str) -> Vec<&mut CompilerOption>;
    fn add_or_modify(&mut self, option: &CompilerOption);
    fn add_or_mix(&mut self, option: &CompilerOption);
    fn add_option(&mut self, other: &CompilerOption);
    fn is_preprocessor(&self) -> bool;
    fn is_compiling(&self) -> bool;
    fn is_checking(&self) -> bool;
}

impl OptionManagement for Vec<CompilerOption> {
    fn get_options(&self, name: &str) -> Vec<&CompilerOption> {
        self.iter()
            .filter(move |opt| opt.name == name && opt.is_enabled)
            .collect()
    }

    fn get_mut_options(&mut self, name: &str) -> Vec<&mut CompilerOption> {
        self.iter_mut()
            .filter(|opt| opt.name == name && opt.is_enabled)
            .collect()
    }

    fn add_or_modify(&mut self, other: &CompilerOption) {
        let mut existed_options = self.get_mut_options(&other.name);
        match existed_options.len() {
            1 => existed_options[0].values.clone_from(&other.values),
            _ => self.push(other.clone()),
        }
    }

    fn add_or_mix(&mut self, other: &CompilerOption) {
        let mut existed_options = self.get_mut_options(&other.name);

        match existed_options.len() {
            1 => {
                let existed_option = &mut existed_options[0];
                if existed_option.has_value {
                    for value in &other.values {
                        existed_option.add_or_update_value(value);
                    }
                } else {
                    existed_option.values.clone_from(&other.values);
                }
            }
            _ => self.push(other.clone()),
        }
    }

    fn add_option(&mut self, other: &CompilerOption) {
        self.push(other.clone());
    }

    fn is_preprocessor(&self) -> bool {
        self.iter().any(|opt| opt.name == "-E" && opt.is_enabled)
    }

    fn is_compiling(&self) -> bool {
        self.iter().any(|opt| opt.name == "-c" && opt.is_enabled)
    }

    /// The compiler has no option that is not starting with `-` shall be considered as a checking process.
    fn is_checking(&self) -> bool {
        // TODO: support response file mode (e.g. `clang @file`). If you don't know what it is, google it.
        self.iter().all(|opt| opt.name.starts_with('-'))
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
