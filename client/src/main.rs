
#[tokio::main]
async fn main(){
    match reqwest::get("http://127.0.0.1:8000")
    .await {
        Ok(resp) => println!("{}", resp.text().await.unwrap()),
        Err(err) => println!("{}", err),
    }
    
}


