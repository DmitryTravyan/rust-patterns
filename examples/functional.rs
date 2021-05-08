use reqwest::{get};
use reqwest::Client;
use futures::executor::block_on;


enum Req {
    Get {
        url: String,
        header: String,
    },
    Post {
        url: String,
        body: String,
    }
}

trait MakeReq<T> {
    fn make_req(body: String) -> Option<String>;
}

impl Req::Post {
    fn make_req(url: String, body: String) -> Option<String> {
        block_on(Client::new().post(url).body(body).send().await.expect("Err").text().await.expect("nok"))
    }
}

impl Req::Get {
    fn make_req(url: String) -> Option<String> {
        block_on(get(url).await.expect("Nok").text().await.expect("Nok"))
    }
}

pub fn main() {

}

