use rscx::{component, html};

#[component]
fn NoArgsComponent() -> String {
    html! {
        <div>"hello"</div>
    }
}

#[tokio::test]
async fn test_component_no_args_direct_call() {
    let result = NoArgsComponent(NoArgsComponentProps::builder().build()).await;
    assert_eq!(result, "<div>hello</div>");
}

#[tokio::test]
async fn test_component_no_args_in_html_macro() {
    let result = html! {
        <NoArgsComponent />
    };
    assert_eq!(result, "<div>hello</div>");
}
