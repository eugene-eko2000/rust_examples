use std::sync::{Arc, RwLock};

use anyhow::{anyhow, Result};
use async_trait::async_trait;

struct ValStruct {
    f1: i32,
}

#[async_trait(?Send)]
pub trait MyTrait {
    async fn my_func(&mut self) -> Result<()>;
}

struct MyStruct {
    val_struct: Arc<RwLock<ValStruct>>,
}

impl MyStruct {
    fn new(v: Arc<RwLock<ValStruct>>) -> Self {
        Self { val_struct: v }
    }
}

#[async_trait(?Send)]
impl MyTrait for MyStruct {
    async fn my_func(&mut self) -> Result<()> {
        let s = self.val_struct.read().map_err(|e| anyhow!("{e}"))?;
        inner_func(s.f1.clone()).await;
        Ok(())
    }
}

async fn inner_func(v: i32) {
    println!("Inner value: {:?}", v);
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut my_struct = MyStruct::new(Arc::new(RwLock::new(ValStruct{f1: 42})));
    my_struct.my_func().await?;
    println!("Hello, world!");
    Ok(())
}
