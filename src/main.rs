use argus::{
    option_manager::CompilerOptionManager,
    option_visitors::{
        AsanVisitor, DefaultOptimizationVisitor, DefaultParametersVisitor, LibfuzzerVisitor,
        OptionVisitor,
    },
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect(); // skip program name

    let mut manager = CompilerOptionManager::new();

    for arg in args {
        manager.add_option(arg, true);
    }

    let visitors: Vec<Box<dyn OptionVisitor>> = vec![
        Box::new(DefaultParametersVisitor),
        Box::new(DefaultOptimizationVisitor),
        Box::new(AsanVisitor),
        Box::new(LibfuzzerVisitor),
    ];

    for visitor in visitors {
        visitor.visit(&mut manager.options);
    }

    manager.print_command();
}
