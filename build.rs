
fn main() {
    #[cfg(feature = "bindings")]
    {
        use ligen::prelude::*;
        use ligen_csharp::CSharpGenerator;

        match Project::current() {
            Ok(project) => {
                println!("Generating C# bindings...");
                let csharp_generator = CSharpGenerator::default();
                csharp_generator.generate(&project).expect("Failed to generate C# bindings.");
            },
            Err(e) => println!("e: {:#?}", e)
        }
    }
}

