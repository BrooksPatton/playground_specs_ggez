use bbggez::ggez::{
	event::EventHandler,
	graphics,
	nalgebra::{Point2, Vector2},
	Context, GameResult,
};
use specs::{Builder, Component, DenseVecStorage, ReadStorage, RunNow, System, World, WorldExt};

pub struct Game {
	arena_width: f32,
	arena_height: f32,
	world: World,
}

impl Game {
	pub fn new() -> Game {
		let arena_width = 10000.0;
		let arena_height = 5000.0;
		let mut world = World::new();
		let grid_spacing = 100.0;

		world.register::<Location>();

		for count in (1..arena_width / grid_spacing) {}

		world
			.create_entity()
			.with(Location::new(50.0, 50.0))
			.build();

		Game {
			arena_width,
			arena_height,
			world,
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
		self.world.maintain();
		Ok(())
	}
	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);
		let mut draw_system = DrawSystem(context);
		draw_system.run_now(&self.world);
		graphics::present(context)
	}
}

#[derive(Component)]
struct Location(Vector2<f32>);

impl Location {
	pub fn new(x: f32, y: f32) -> Location {
		Location(Vector2::new(x, y))
	}
}

struct DrawSystem<'a>(&'a mut Context);

impl<'a> System<'a> for DrawSystem<'a> {
	type SystemData = (ReadStorage<'a, Location>);

	fn run(&mut self, location: Self::SystemData) {
		use specs::Join;

		for location in (&location).join() {
			let mesh = graphics::MeshBuilder::new()
				.circle(
					graphics::DrawMode::fill(),
					Point2::from(location.0),
					10.0,
					0.1,
					graphics::WHITE,
				)
				.build(self.0)
				.unwrap();

			graphics::draw(self.0, &mesh, graphics::DrawParam::default()).unwrap();
		}
	}
}
