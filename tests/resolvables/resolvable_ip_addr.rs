use rsip_dns::resolvables::*;

#[tokio::test]
async fn resolves_correctly() {
    use testing_utils::Randomize;

    let mut resolvable =
        ResolvableIpAddr::new(Randomize::random(), Randomize::random(), Randomize::random());

    assert!(resolvable.resolve_next().await.is_some());
    assert!(resolvable.resolve_next().await.is_none());
}
