// fn main() -> Result<(), Box<dyn std::error::Error>> {
// tonic_build::compile_protos("proto/echo.proto")?;
// Ok(());
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .out_dir("proto/")
        .compile(&["proto/echo.proto"], &["proto/"])?;
    Ok(())
}
