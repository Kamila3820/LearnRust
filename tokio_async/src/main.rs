use tokio;

async fn gather_herbs() {
    println!("Gathering Herbs...");
}

async fn collect_coin() {
    println!("Collecting Coins!");
}

async fn fight_monster() {
    println!("Fighting Enemy!!");
}

#[tokio::main]
async fn main() {
    let herbs = tokio::spawn(gather_herbs());
    let coin = tokio::spawn(collect_coin());
    let mons = tokio::spawn(fight_monster());

    let _ = tokio::join!(herbs, coin, mons);
}
