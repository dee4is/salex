use std::io::Result;
fn main() -> Result<()> {
    prost_build::compile_protos(
        &[
            "organization.proto",
            "customer.proto",
            "manager.proto",
            "order.proto",
            "product.proto",
            "warehouse.proto",
        ],
        &["proto/"],
    )?;
    Ok(())
}
