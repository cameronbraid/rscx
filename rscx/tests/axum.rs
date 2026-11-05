#![cfg(feature = "axum")]

use axum::{
    body::to_bytes,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use rscx::{context, html};
use std::rc::Rc;

async fn page() -> Html<String> {
    rscx::axum::render(async {
        context::provide_context(Rc::new(String::from("hello")));
        tokio::task::yield_now().await;
        let greeting = context::expect_context::<Rc<String>>();
        html! { <div>{greeting.as_str()}</div> }
    })
    .await
}

#[tokio::test]
async fn renders_an_axum_html_response_from_a_handler() {
    // Registering the handler checks Axum's Handler and Send requirements.
    let _router = Router::<()>::new().route("/", get(page));
    let response = page().await.into_response();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers()[CONTENT_TYPE], "text/html; charset=utf-8");
    let body = to_bytes(response.into_body(), 1024).await.unwrap();
    assert_eq!(body.as_ref(), b"<div>hello</div>");
}

#[tokio::test]
async fn renders_concurrently_with_isolated_local_contexts() {
    let renders = (0usize..8).map(|value| {
        rscx::axum::render(async move {
            assert!(context::use_context::<Rc<usize>>().is_none());
            context::provide_context(Rc::new(value));
            tokio::task::yield_now().await;
            context::spawn_local(async {
                let value = Rc::new(String::from("local child"));
                tokio::task::yield_now().await;
                assert_eq!(value.as_str(), "local child");
            })
            .await
            .unwrap();
            *context::expect_context::<Rc<usize>>()
        })
    });

    let values: Vec<_> = futures::future::join_all(renders)
        .await
        .into_iter()
        .map(|Html(value)| value)
        .collect();
    assert_eq!(values, (0..8).collect::<Vec<_>>());
}
