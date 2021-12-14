
fn main() {
    #[cfg(feature = "bindings")]
    {
        use ligen::prelude::*;
        use ligen_csharp::CSharpGenerator;
        use ligen_cargo::CargoProject;
        use ligen_rust::RustGenerator;

        match CargoProject::current().and_then(Project::try_from) {
            Ok(project) => {
                println!("Generating C# bindings...");
                let rust_generator = RustGenerator::default();
                let csharp_generator = CSharpGenerator::default();
                rust_generator.generate(&project).expect("Failed to generate Rust interface.");
                csharp_generator.generate(&project).expect("Failed to generate C# interface.");
            },
            Err(e) => panic!("e: {:#?}", e)
        }
    }
}

