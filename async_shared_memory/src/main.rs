use std::{
    collections::HashMap, sync::{Arc, Mutex}
};

use anyhow::{anyhow, Result};
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let data = Arc::new(Mutex::new(HashMap::<String, u32>::new()));
    let mut js = JoinSet::<Result<()>>::new();

    for i in 0..10 {
        let data_copy = Arc::clone(&data);
        js.spawn(async move {
            for j in 0..1000 {
                if j % 10 == i {
                    let mut d = data_copy.lock().map_err(|e| anyhow!("Lock error: {}", e))?;
                    let key = format!("k{}", i);
                    if let Some(&mut val) = d.get_mut(&key) {
                        d.insert(key, val + j);
                    }
                    else {
                        d.insert(key, j);
                    }
                }
            }
            Ok(())
        });
    }
    let _ = js.join_all().await;
    println!("{:#?}", data.lock());
}
