use rscx::{CollectFragment, CollectFragmentAsync, MapFragmentAsyncExt, MapFragmentExt};

#[test]
fn synchronous_fragment_helpers_preserve_order() {
    let collected = ["<b>first</b>", "<i>second</i>"]
        .into_iter()
        .map(String::from)
        .collect_fragment();
    assert_eq!(collected, "<b>first</b><i>second</i>");

    let mapped = [1, 2, 3].into_iter().map_fragment(|value| value);
    assert_eq!(mapped, "123");

    assert_eq!(std::iter::empty::<String>().collect_fragment(), "");
    assert_eq!(std::iter::empty::<usize>().map_fragment(|value| value), "");
}

#[tokio::test]
async fn asynchronous_fragment_helpers_preserve_order() {
    let collected = (0..3)
        .map(|value| async move {
            for _ in value..3 {
                tokio::task::yield_now().await;
            }
            value.to_string()
        })
        .collect_fragment_async()
        .await;
    assert_eq!(collected, "012");

    let mapped = vec![1, 2, 3]
        .map_fragment(|value| async move {
            tokio::task::yield_now().await;
            value
        })
        .await;
    assert_eq!(mapped, "123");

    let empty = Vec::<usize>::new()
        .map_fragment(|value| async move { value })
        .await;
    assert_eq!(empty, "");
}
