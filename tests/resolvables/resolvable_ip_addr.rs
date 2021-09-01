use rsip_dns::*;

#[tokio::test]
async fn resolves_correctly() {
    use rsip::Randomize;

    let mut resolvable = ResolvableIpAddr::new(
        Randomize::random(),
        Randomize::random(),
        Randomize::random(),
    );

    assert!(resolvable.resolve_next().await.is_some());
    assert!(resolvable.resolve_next().await.is_none());
}
