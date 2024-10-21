use crate::{
    compiler_option::{CompilerOption, OptionManagement},
    env::ADD_ADDITIONAL_OBJECTS,
};

use super::OptionVisitor;

#[derive(Default)]
pub struct AdditionalObjectsVisitor {}

impl OptionVisitor for AdditionalObjectsVisitor {
    fn visit(&mut self, options: &mut Vec<CompilerOption>) {
        let object_list =
            std::env::var(ADD_ADDITIONAL_OBJECTS).unwrap_or_else(|_| String::from(""));
        for object in object_list.split(',') {
            options.add_or_modify(&CompilerOption::new(object));
        }
    }
}
