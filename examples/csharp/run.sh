cargo build --manifest-path ../../bindings/csharp/lib/Cargo.toml
cp ../../bindings/csharp/lib/target/debug/libopenlimits_sharp.so ./bin/Debug/netcoreapp5.0/
dotnet run

