#![allow(unused)]

use pin_project_lite::pin_project;
use std::future::Future;
use std::pin;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::Sleep;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;

#[tokio::main]
async fn main() {
    println!("async ..");
    exec().await;
    println!("end async..");
}

struct Task {}

impl Drop for Task {
    fn drop(&mut self) {
        println!("drop task..");
    }
}

impl Future for Task {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("inner task...");
        unsafe {
            println!("COUNT: {COUNT}");
        }
        unsafe {
            COUNT += 1;
        }
        // let this = self.project();
        // this.inner.poll(cx)
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

pin_project! {
    struct MyTask{
        #[pin]
        inner :Task,
    }
}

impl Future for MyTask {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("exec my task");

        // let task = Task {
        //     // inner: tokio::time::sleep(Duration::from_secs(5)),
        // };
        //
        // std::pin::pin!(task).poll(cx);
        let this = self.project();
        this.inner.poll(cx);
        if unsafe { COUNT == 2 } {
            println!("1111");
            Poll::Ready(())
        } else {
            println!("222");
            Poll::Pending
        }

        // let this = self.project();
        // match this.inner.poll(cx) {
        //     Poll::Pending => {
        //         println!("pending task");
        //         Poll::Pending
        //     }
        //     Poll::Ready(_x) => {
        //         println!("end exec task");
        //         Poll::Ready(())
        //     }
        // }
    }
}

pin_project! {
    struct WrapTask{
        #[pin]
        inner: MyTask,
    }
}

impl Future for crate::WrapTask {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("wrap task...");
        let this = self.project();
        this.inner.poll(cx)
    }
}

static mut COUNT: i8 = 0;

async fn exec() {
    // println!("kd");
    // let my = MyTask { inner: Task {
    //     inner:tokio::time::sleep(Duration::from_secs(5)),
    // } };
    // my.await;
    println!("exec func wrap task");

    let my1 = MyTask {
        inner: Task {
            // inner: tokio::time::sleep(Duration::from_secs(5)),
        },
    };
    let w = WrapTask { inner: my1 };
    w.await;
}

async fn serve() {
    use axum::{routing::get, Router};

    let router = Router::new()
        .route(
            "/",
            get(|| async {
                println!("exec handler");
                "Hello, World!"
            }),
        )
        .layer(CorsLayer::new())
        .layer(TimeoutLayer::new(Duration::from_secs(5)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
