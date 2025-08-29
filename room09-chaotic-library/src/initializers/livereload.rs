use async_trait::async_trait;
use axum::Router as AxumRouter;
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};
use tower_livereload::LiveReloadLayer;

pub struct LiveReloadInitializer;

#[async_trait]
impl Initializer for LiveReloadInitializer {
    fn name(&self) -> String {
        "livereload".to_string()
    }

    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        if cfg!(debug_assertions) {
            Ok(router.layer(LiveReloadLayer::new()))
        } else {
            Ok(router)
        }
    }
}
