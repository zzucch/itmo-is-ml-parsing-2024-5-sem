use std::{sync::Arc, time::Duration};

use anyhow::{bail, Context, Result};
use headless_chrome::{Browser, Tab};
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

pub async fn get_page_data_chrome(browser: &Browser, url: &str) -> Result<(String, String)> {
    let tab = browser.new_tab().context("Failed to create new tab")?;

    tab.navigate_to(url).context("Failed to navigate to URL")?;
    tab.wait_until_navigated()
        .context("Failed to wait until navigated")?;

    wait_for_element(&tab, "head", Duration::from_secs(10)).await?;
    wait_for_element(&tab, "body", Duration::from_secs(10)).await?;

    let head_content = tab
        .evaluate("document.head.outerHTML", true)
        .context("Failed to evaluate head content")?
        .value
        .context("Failed to get value from evaluation")?
        .as_str()
        .context("Failed to convert evaluation value to string")?
        .to_string();

    let body_content = tab.get_content().context("Failed to get body content")?;

    close_tab_with_retry(tab)?;

    Ok((head_content, body_content))
}

async fn wait_for_element(tab: &Tab, element: &str, duration: Duration) -> Result<()> {
    let wait_for_element = async {
        tab.wait_for_element(element)
            .context(format!("Failed to wait for {} element", element))?;
        Ok(())
    };

    match timeout(duration, wait_for_element).await {
        Ok(result) => result,
        Err(_) => bail!("Timed out waiting for {} element", element),
    }
}

fn close_tab_with_retry(tab: Arc<Tab>) -> Result<()> {
    for _i in 0..10 {
        if let Ok(_) = tab.close(false) {
            return Ok(());
        }
    }

    tab.close(false).context("Failed to close tab")?;

    Ok(())
}
