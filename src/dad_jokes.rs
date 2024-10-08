pub async fn fetch() -> String {
    reqwest::Client::new().get("https://icanhazdadjoke.com/")
        .header("User-Agent", "Metal Pipe Discord bot (https://github.com/funnsam/metal_pipe)")
        .header("Accept", "text/plain")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
