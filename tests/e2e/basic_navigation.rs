use playwright::api::{Browser, BrowserType, Page};
use playwright::Playwright;
use std::process::{Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_basic_navigation() -> Result<(), Box<dyn std::error::Error>> {
    // Start production server
    let mut server = Command::new("cargo")
        .args(["run", "--release", "--features", "frontend"])
        .stdout(Stdio::null())
        .spawn()?;

    // Wait for server startup
    sleep(Duration::from_secs(3)).await;

    // Configure Playwright
    let playwright = Playwright::initialize().await?;
    let chromium = playwright.chromium();
    let browser = chromium.launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;

    // Execute test steps
    page.goto("http://localhost:52389").await?;
    assert!(page.title().await?.contains("WishApp"));
    
    // Teardown
    server.kill()?;
    Ok(())
}