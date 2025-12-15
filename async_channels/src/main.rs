use std::collections::HashMap;

use anyhow::{anyhow, Result};
use tokio::{spawn, task::JoinSet};

#[tokio::main]
async fn main() -> Result<()>{
    let (tx, rx) = async_channel::bounded(32);
    let mut js = JoinSet::<Result<()>>::new();
    for i in 0..10 {
        let tx = tx.clone();
        js.spawn(async move {
            for j in 0..1000 {
                if j % 10 == i {
                    tx.send((i, j)).await.map_err(|e| anyhow!("Send error: {:?}", e))?;
                }
            }
            Ok(())
        });
    }

    let h = spawn(async move {
        let mut data = HashMap::<String, u32>::new();
        loop {
            let res = rx.recv().await;
            match res {
                Ok((vi, vj)) => {
                    let key = format!("k{}", vi);
                    if let Some(v) = data.get(&key) {
                        data.insert(key, v + vj);
                    } else {
                        data.insert(key, vj);
                    }
                }
                Err(_) => { break; }
            }
        }
        println!("{:#?}", data);
    });

    js.join_all().await;
    drop(tx);
    h.await?;

    Ok(())
}
