
use std::future::Future;
use std::pin::Pin;

struct ReadyFuture<T> {
    value: Option<T>,
}

impl<T> Future for ReadyFuture<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut std::task::Context<'_>) -> std::task::Poll<T> {
        let this = unsafe { self.get_unchecked_mut() };

        std::task::Poll::Ready(
            this.value
                .take()
                .expect("ReadyFuture polled after completion")
        )
    }
}


async fn add(a: i32, b: i32) -> i32 {
    return a+b;
}

async fn compute(x: i32) -> i32 {

    let result = add(x,x).await;

    let final_resut = add(result , 1).await;




    return final_resut;
}


fn make_greeter(name: String) -> impl Future<Output = String> {
    async move { format!("Hello, {name}!") }
}

async fn parse_num(s: &str) -> i32 {
    return s.parse::<i32>().unwrap();
}

async fn double(n: i32) -> i32 {
    return n*2;
}

async fn to_message(n: i32) -> String {
    return format!("Result: {n}")
}

async fn process(input: &str) -> String {
    let result = parse_num(input).await;
      let result = double(result).await;

      let result = to_message(result).await;

  return result;
}