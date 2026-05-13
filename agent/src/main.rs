use notify::{Watcher, RecursiveMode, Config};
use std::path::Path;
use std::sync::mpsc::channel;

#[tokio::main]
async fn main() -> notify::Result<()> {
    let path_to_watch = "/data/data/com.termux/files/home/test_security.txt"; 

    println!("🛡️ SystemGuard Agent starting...");
    let (tx, rx) = channel();
    let mut watcher = notify::RecommendedWatcher::new(tx, Config::default())?;

    if Path::new(path_to_watch).exists() {
        watcher.watch(Path::new(path_to_watch), RecursiveMode::NonRecursive)?;
        println!("✅ Monitoring active on {}", path_to_watch);
    }

    println!("Waiting for events... (Telegram alerts are ENABLED)");
    
    // Blocking loop ko thread mein chalayein taake async context kharab na ho
    tokio::task::spawn_blocking(move || {
        for res in rx {
            match res {
                Ok(event) => {
                    let msg = format!("⚠️ SECURITY ALERT: Activity on SystemGuard! {:?}", event);
                    println!("{}", msg);
                    
                    // Simple blocking call inside blocking thread
                    let _ = send_telegram_alert(&msg);
                },
                Err(e) => println!("Watcher error: {:?}", e),
            }
        }
    }).await.unwrap();

    Ok(())
}

fn send_telegram_alert(message: &str) -> reqwest::Result<()> {
    let token = "6527105958:AAEddIn493VLJYXPD0ftCBdEJ3yAd3ZJlWo";
    let chat_id = "1155701446";
    
    let url = format!(
        "https://api.telegram.org/bot{}/sendMessage?chat_id={}&text={}", 
        token, chat_id, message
    );
    
    // Blocking client ko loop ke bahar ya yahan initialize karein
    let client = reqwest::blocking::Client::new();
    client.get(url).send()?;
    Ok(())
}













