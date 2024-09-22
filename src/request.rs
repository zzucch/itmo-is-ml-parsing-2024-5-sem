pub async fn get_body(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;

    let body = response.text().await?;

    Ok(body)
}
