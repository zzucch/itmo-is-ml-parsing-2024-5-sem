use std::time::Duration;

use anyhow::{anyhow, Context, Result};
use headless_chrome::Browser;
use reqwest;
use tokio::time::timeout;

pub async fn get_body(url: &str) -> Result<String> {
    let response = reqwest::get(url).await.context("Failed to get URL")?;

    let body = response
        .text()
        .await
        .context("Failed to get response text")?;

    Ok(body)
}

pub async fn get_head_chrome(browser: &Browser, url: &str) -> Result<String> {
    let tab = browser.new_tab().context("Failed to create new tab")?;

    tab.navigate_to(url).context("Failed to navigate to URL")?;
    tab.wait_until_navigated()
        .context("Failed to wait until navigated")?;

    let wait_for_head = async {
        tab.wait_for_element("head")
            .context("Failed to wait for head element")
    };

    match timeout(Duration::from_secs(10), wait_for_head).await {
        Ok(result) => result?,
        Err(_) => return Err(anyhow!("Timed out waiting for head element")),
    };

    let head_content = tab
        .evaluate("document.head.outerHTML", true)
        .context("Failed to evaluate head content")?
        .value
        .context("Failed to get value from evaluation")?
        .as_str()
        .context("Failed to convert evaluation value to string")?
        .to_string();

    tab.close(false).context("Failed to close tab")?;

    Ok(head_content)
}
