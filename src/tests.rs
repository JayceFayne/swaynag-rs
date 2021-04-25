use crate::{Button, Edge, Swaynag};

#[cfg(not(feature = "async"))]
#[test]
fn spwan() {
    let mut child = Swaynag::new("swaynag test from rust")
        .detailed_message("interesting details")
        .button(Button::detailed("show me more"))
        .button(Button::simple("simple", "false"))
        .button(Button::dismiss("go away", "yes"))
        .button(Button::override_default_dismiss("Y"))
        .edge(Edge::Bottom)
        .spawn()
        .unwrap();
    child.wait().unwrap();
}

#[cfg(feature = "async")]
#[test]
fn spwan_async() {
    async_io::block_on(async {
        let mut child = Swaynag::new("swaynag test from rust")
            .detailed_message("interesting details")
            .button(Button::detailed("show me more"))
            .button(Button::simple("simple", "false"))
            .button(Button::dismiss("go away", "yes"))
            .button(Button::override_default_dismiss("Y"))
            .edge(Edge::Bottom)
            .spawn()
            .await
            .unwrap();
        child.wait().await.unwrap();
    })
}
