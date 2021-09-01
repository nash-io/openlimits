use ligen::prelude::*;
// use ligen_c::CGenerator;
// use ligen_cmake::{CMakeGenerator, Language};
use ligen_csharp::CSharpGenerator;
//use ligen::ir::Module;

fn main() {
    // let module = Module::from_path("src/model/candle.rs").expect("Couldn't read module.");
    // if let Ok(project) = Project::read_from_module(module) {
    if let Ok(project) = Project::current() {
        // let c_generator = CGenerator::default();
        // let cmake_generator = CMakeGenerator(Language::C);
        let csharp_generator = CSharpGenerator::default();
        // cmake_generator.generate(&project).expect("Couldn't generate CMake project.");
        // c_generator.generate(&project).expect("Couldn't generate C bindings.");
        csharp_generator.generate(&project).expect("Couldn't generate C# bindings.")
    }
}