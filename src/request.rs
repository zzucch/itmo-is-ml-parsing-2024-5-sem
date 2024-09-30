use headless_chrome::{Browser, LaunchOptionsBuilder};

pub async fn get_body(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;

    let body = response.text().await?;

    Ok(body)
}

pub async fn get_body_chrome(
    browser: &Browser,
    url: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let tab = browser.new_tab()?;

    tab.navigate_to(url)?;
    tab.wait_for_element("body")?;

    let body = tab.get_content()?;

    tab.close(true)?;

    Ok(body)
}
