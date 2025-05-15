use asteroids::{
	elements::{shape, Lines, Spatial},
	ClientState, ElementTrait, Migrate,
};
use serde::{Deserialize, Serialize};
use stardust_xr_fusion::{fields::Shape, project_local_resources};

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
	const QUALIFIER: &'static str = "org";
	const ORGANIZATION: &'static str = "example";
	const NAME: &'static str = "asteroids_test";

	fn reify(&self) -> asteroids::Element<Self> {
		Lines::new(shape(Shape::Box([0.1; 3].into()))).build()
	}
}
