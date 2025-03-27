use playwright::Playwright;
use std::time::Duration;

#[tokio::test]
async fn test_counter_increment() -> Result<(), Box<dyn std::error::Error>> {
    let playwright = Playwright::initialize().await?;
    let browser = playwright.chromium().launcher().headless(true).launch().await?;
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;

    // Navigate to the application
    page.goto_builder("http://localhost:52389").goto().await?;

    // Verify the initial counter value
    let initial_counter = page.inner_text("p", None).await?;
    println!("Initial Counter: {}", initial_counter);

    // Click the increment button
    page.click_builder("button").click().await?;

    // Verify the counter value after click
    let updated_counter = page.inner_text("p", None).await?;
    println!("Updated Counter: {}", updated_counter);

    assert_ne!(initial_counter, updated_counter);

    browser.close().await?;
    Ok(())
}

