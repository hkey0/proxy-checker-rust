use std::fs;
use std::time::Duration;

#[async_std::main]
async fn main() {
    let file = fs::read_to_string("files/proxies.txt")
        .expect("Dosya okunamadı")
        .replace("\"", "");    
    
    let replaced = file.replace("\r", "");
    let proxy_list = replaced.split("\n").collect::<Vec<&str>>();

    for wproxy in proxy_list {
        match check_proxy(wproxy.to_string()).await{
            true => {
                println!("Checked => {wproxy}: LIVE");
                let mylove = fs::read_to_string("files/live.txt")
                    .expect("Dosya okunamadı");
                
                fs::write("files/live.txt", format!("{mylove}{wproxy}\r\n"))
                    .expect("Yazılamadı");          
            }
            false => {
                println!("Checked => {wproxy}: DEAD");
                let mylove = fs::read_to_string("files/dead.txt")
                    .expect("Dosya okunamadı");
                fs::write("files/dead.txt", format!("{mylove}{wproxy}\r\n"))
                    .expect("Yazılamadı");
                
            }
        }
    }
}


async fn check_proxy(proxys: String) -> bool{
    print!("{:#?}", proxys);

    match reqwest::Proxy::https(format!("socks5://{proxys}", proxys=proxys)) {
        Ok(maproxy) => {
            let client = reqwest::Client::builder()
                .proxy(maproxy)
                .build().unwrap();
            
            match client.get("https://wtfismyip.com/json")
                .timeout(Duration::from_secs(5))
                .send()
                .await
                {
                    Ok(_) => {
                        return true;
                    }
                    Err(_) => {
                        return false;
                    }
                };
        }
        Err(_) => {
            return false;
        }

        _ => {
            return false;
        }
    };
}
