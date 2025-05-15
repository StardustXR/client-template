use asteroids::{elements::Model, ClientState, ElementTrait, Migrate};
use serde::{Deserialize, Serialize};
use stardust_xr_fusion::project_local_resources;

#[tokio::main(flavor = "current_thread")]
async fn main() {
	asteroids::client::run::<State>(&[&project_local_resources!("res")]).await
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct State {}
impl Migrate for State {
	type Old = Self;
}
impl ClientState for State {
	const APP_ID: &'static str = "org.example.client_template";

	fn reify(&self) -> asteroids::Element<Self> {
		Model::namespaced("template", "color_cube").build()
	}
}
