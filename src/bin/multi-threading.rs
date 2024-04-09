use std::time::Duration;
use tokio::{task::{JoinError, JoinSet}, time::sleep};
use rand::{thread_rng, Rng};

#[tokio::main]
async fn main() -> Result<(), JoinError>{
    println!("Hello, world!");
    let mut jhs = JoinSet::new();
    for i in 1..=20 {
        jhs.spawn(fact(i));
    }
    while let Some(res) = jhs.join_next().await {
        let (n, r) = res?;
        println!("{}! = {}", n, r);
    }
    Ok(())
}

async fn fact(n: u32) -> (u32, u128) {
    let p = thread_rng().gen_bool(0.5);
    if p {
        let ms = thread_rng().gen_range(500..2000) as u64;
        sleep(Duration::from_millis(ms)).await;
    }
    let mut res: u128 = 1;
    for i in 2..=n {
        res *= i as u128;

    }
    (n, res)
}
