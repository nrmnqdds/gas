fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/unaryecho/echo.proto")?;
    tonic_prost_build::compile_protos("proto/auth/auth.proto")?;
    Ok(())
}
