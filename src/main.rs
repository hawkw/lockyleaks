use futures::{future, stream::futures_unordered::FuturesUnordered};
use pin_project::pin_project;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::{
    stream::StreamExt,
    sync::{Mutex, MutexGuard},
};

#[tokio::main]
async fn main() {
    let lock = Arc::new(Mutex::new(()));
    let mut funordered = (0..10_000)
        .map(|i| {
            let lock = lock.clone();
            let f = tokio::spawn(async move {
                let lock = lock.clone();
                for _ in 0..109 {
                    let lock2 = lock.clone();
                    tokio::time::timeout(Duration::from_millis(100), async move {
                        let guard = lock2.lock().await;
                        future::pending::<()>().await;
                        drop(guard);
                    })
                    .await;
                }
            });
            // println!("spawned {}", i);
            f
        })
        .collect::<FuturesUnordered<_>>();
    while let Some(_) = funordered.next().await {
        // println!("future finished")
    }
}
