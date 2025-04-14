use std::future::Future;

#[tokio::main]
async fn main() {
    println!("main() {} thread id: {:?}", line!(), std::thread::current().id());
    async_call().await;

    println!("main() {} thread id: {:?}", line!(), std::thread::current().id());
    de_async_call().await;

    println!("main() {} thread id: {:?}", line!(), std::thread::current().id());
    println!("Hello, world!");
}

#[inline(never)]
async fn async_call() {
    println!("async_call {} thread id: {:?}", line!(), std::thread::current().id());
    SLEEP().await;
    println!("async_call {} thread id: {:?}", line!(), std::thread::current().id());
}

#[inline(never)]
fn de_async_call() -> impl Future<Output = ()> {
    println!("de_async_call {} thread id: {:?}", line!(), std::thread::current().id());
    let x = SLEEP();
    println!("de_async_call {} thread id: {:?}", line!(), std::thread::current().id());
    x
}

#[inline(never)]
async fn SLEEP() {
    // print thread id
    println!("sleep {} thread id: {:?}", line!(), std::thread::current().id());
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    // print thread id
    println!("sleep {} thread id: {:?}", line!(), std::thread::current().id());
}