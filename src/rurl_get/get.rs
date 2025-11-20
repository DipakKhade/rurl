use reqwest;

pub async fn get(url: String) -> String {
    println!("get req to -- {}", url);
    let res = reqwest::get(url).await.expect("req failed").text().await.expect("req failed");
    res
}