use std::error::Error;
use aeon_core::network::service::NetworkService;

#[tokio::main]
// FIX: We align the return type with the Service's error type (Send + Sync)
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    
    // دریافت سرویس و کانال فرماندهی
    let (mut service, _cmd_tx) = NetworkService::new(12345).await?;

    let addr = "/ip4/0.0.0.0/tcp/0".parse()?;
    service.listen(addr)?;

    println!("Aeon Node is running...");
    
    // ورود به حلقه اصلی
    service.run().await;

    Ok(())
}
