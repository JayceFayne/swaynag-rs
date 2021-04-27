use crate::Swaynag;

#[cfg(not(feature = "async"))]
#[test]
fn spwan() {
    let mut child = Swaynag::new("swaynag test from rust")
        .details_button("show me more")
        .button("simple", "false")
        .dismiss_button("go away", "true")
        .override_default_dismiss_button("Y")
        .display_on_bottom_edge()
        .spawn_with_detailed_message("interesting details")
        .unwrap();
    child.wait().unwrap();
}

#[cfg(feature = "async")]
#[test]
fn spwan() {
    async_io::block_on(async {
        let mut child = Swaynag::new("swaynag test from rust")
            .details_button("show me more")
            .button("simple", "false")
            .dismiss_button("go away", "true")
            .override_default_dismiss_button("Y")
            .display_on_bottom_edge()
            .spawn_with_detailed_message("interesting details")
            .await
            .unwrap();
        child.wait().await.unwrap();
    })
}
