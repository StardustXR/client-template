use std::f32::consts::FRAC_PI_2;

use asteroids::{
	elements::{shape, Dial, GrabRing, Lines},
	ClientState, CustomElement, Migrate, Reify, Transformable,
};
use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};
use stardust_xr_fusion::{fields::Shape, project_local_resources};

#[tokio::main(flavor = "current_thread")]
async fn main() {
	asteroids::client::run::<State>(&[&project_local_resources!("res")]).await
}

#[derive(Debug, Serialize, Deserialize)] // Defining variables used in client
pub struct State {
	grab_pos: Vec3,
	time: f32,
	cube_edge_length: f32,
}

impl Default for State {
	// Defines the default value for the variables used in the client
	fn default() -> Self {
		Self {
			grab_pos: Default::default(),
			time: Default::default(),
			cube_edge_length: 0.1,
		}
	}
}
impl Migrate for State {
	type Old = Self;
}
impl ClientState for State {
	const APP_ID: &'static str = "org.example.client_template";

	fn on_frame(&mut self, _info: &stardust_xr_fusion::root::FrameInfo) {
		self.time += _info.delta;
	} // scale before identical scale to when it reloads
}
impl Reify for State {
	// Example: Cube with grab ring on the bottom (allows user to move it)
	// and a dial on the top that allows the user to change the size of the box
	fn reify(&self) -> impl asteroids::Element<Self> {
		GrabRing::new(self.grab_pos, |state: &mut Self, pos| {
			// Creates Grabbable ring underneath box
			state.grab_pos = pos.into();
		})
		.radius(self.cube_edge_length.hypot(self.cube_edge_length) / 2.0 + 0.025) // Defines radius of grab ring
		.build()
		.child(
			// Cube lines
			Lines::new(shape(Shape::Box([self.cube_edge_length; 3].into()))) // Creates box outline size
				.pos([0.0, self.cube_edge_length / 2.0, 0.0]) // Updates position of box
				.build()
				.child(
					// Dial
					Dial::create(self.cube_edge_length, |state: &mut Self, value: f32| {
						state.cube_edge_length = value.clamp(0.05, 0.9); // Restrict size between 0.05 &
						                               // 0.9 meters
					})
					.turn_unit_amount(0.2) // Defines how much 1 rotation will add or subtract from cube_edge_length
					.radius(0.05) // Defines the radius of the dial
					.thickness(0.04) // Defines the thickness of the dial
					.pos([0.0, self.cube_edge_length / 2.0, 0.0]) // Sets position of rotation of dial (on to of the cube)
					.rot(Quat::from_rotation_x(-FRAC_PI_2)) // Sets orientation to correct plane
					.build(), // Builds the object
				),
		) // Makes the box become a child of the ring
	}
}
