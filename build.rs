fn main() {
    #[cfg(feature = "bindings")]
    {
        use ligen::prelude::*;
        use ligen::traits::build::{BuildSystem, BuildProfile};
        use ligen_csharp::CSharpGenerator;
        use ligen_cargo::{CargoProject, CargoGenerator, CargoBuilder};
        use ligen_rust::RustGenerator;

        match CargoProject::current().and_then(Project::try_from) {
            Ok(project) => {
                println!("Generating C# bindings...");
                let rust_generator = RustGenerator::default();
                let csharp_generator = CSharpGenerator::default();
                let cargo_generator = CargoGenerator::default();
                let cargo_builder = CargoBuilder;
                cargo_generator.generate(&project).expect("Failed to generate Cargo interface.");
                rust_generator.generate(&project).expect("Failed to generate Rust interface.");
                csharp_generator.generate(&project).expect("Failed to generate C# interface.");
                cargo_builder.build(&project, BuildProfile::Release).expect("Failed to build Cargo project.");
            },
            Err(_) => ()
        }
    }
}

