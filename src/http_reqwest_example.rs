use reqwest::blocking::{Client, ClientBuilder};

pub fn reqwest_example(){

    let http_client = Client::new();
    let http_result = http_client.get("https://www.trevorsullivan.net/").send();

    if http_result.is_ok() {
        println!("resuult is {:#?}", http_result.ok().unwrap().text().unwrap());
    }
    else if http_result.is_err() {
        println!("error encountered, error: {:#?}", http_result.err());
    }

    let post_result = http_client.post("http://localhost:3000/submit")
        .header("Authorization", "xxx")
        .body("\"name\": \"pprog\"").send();
    
    if post_result.is_ok(){
        println!("post result is {:#?}", post_result.ok().unwrap().text().unwrap());
    }
    else if post_result.is_err() {
        println!("error encountered, error: {:#?}", post_result.err().unwrap());
    }
}
