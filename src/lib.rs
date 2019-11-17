use bbggez::ggez::{
	event::EventHandler,
	graphics,
	nalgebra::{Point2, Vector2},
	timer::{self, delta, duration_to_f64}, Context, GameResult,
};
use specs::{
	Builder, Component, DenseVecStorage, Read,
	ReadStorage, RunNow, System, World, WorldExt, Write, WriteStorage,
};

pub struct Game {
	arena_width: f32,
	arena_height: f32,
	world: World,
	entities_count: usize,
}

impl Game {
	pub fn new() -> Game {
		let arena_width = 10000.0;
		let arena_height = 5000.0;
		let mut world = World::new();
		let grid_spacing = 25.0;
		let mut entities_count = 0;

		world.register::<Location>();
		world.register::<Size>();
		world.register::<Color>();
    world.register::<Velocity>();

    world.insert(Camera::new());
    world.insert(DeltaTime(0.016));

		for count_width in 1..(arena_width / grid_spacing) as usize {
			for count_height in 1..(arena_height / grid_spacing) as usize {
				world
					.create_entity()
					.with(Location::new(
						count_width as f32 * grid_spacing,
						count_height as f32 * grid_spacing,
					))
					.with(Size::new(3.0))
					.with(Color::new(1.0, 1.0, 1.0, 0.3))
					.build();

				entities_count += 1;
			}
		}

		world
			.create_entity()
			.with(Location::new(500.0, 500.0))
			.with(Size::new(15.0))
			.with(Color::new(1.0, 1.0, 0.0, 1.0))
			.with(Velocity::new(10.0, 1.0))
			.build();

		entities_count += 1;

		Game {
			arena_width,
			arena_height,
			world,
			entities_count,
		}
	}
}

impl EventHandler for Game {
	fn update(&mut self, context: &mut Context) -> GameResult<()> {
    {
      let dt = duration_to_f64(delta(context));
      let mut delta = self.world.write_resource::<DeltaTime>();
      *delta = DeltaTime(dt as f32);
    }

    let mut move_system = MoveSystem;
    move_system.run_now(&self.world);

		self.world.maintain();
		Ok(())
	}
	fn draw(&mut self, context: &mut Context) -> GameResult<()> {
		graphics::clear(context, graphics::BLACK);
		let mut draw_system = DrawSystem(context);
		draw_system.run_now(&self.world);

		let text = graphics::Text::new(format!(
			"entities: {} fps: {}",
			self.entities_count,
			timer::fps(context)
		));

		graphics::draw(
			context,
			&text,
			graphics::DrawParam::default().dest(Point2::new(10.0, 10.0)),
		)?;
		graphics::present(context)
	}
}

#[derive(Default)]
struct DeltaTime(f32);

#[derive(Component)]
struct Location(Vector2<f32>);

impl Location {
	pub fn new(x: f32, y: f32) -> Location {
		Location(Vector2::new(x, y))
	}
}

#[derive(Component)]
struct Size(f32);

impl Size {
	pub fn new(size: f32) -> Size {
		Size(size)
	}
}

#[derive(Component)]
struct Color(graphics::Color);

impl Color {
	pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Color {
		Color(graphics::Color::new(red, green, blue, alpha))
	}
}

#[derive(Default)]
struct Camera {
	x: f32,
	y: f32,
	width: f32,
	height: f32,
}

impl Camera {
	pub fn new() -> Camera {
		let x = 0.0;
		let y = 0.0;
		let width = 1024.0;
		let height = 768.0;

		Camera {
			x,
			y,
			width,
			height,
		}
	}
}

#[derive(Component)]
struct Velocity(Vector2<f32>);

impl Velocity {
	pub fn new(x: f32, y: f32) -> Velocity {
		Velocity(Vector2::new(x, y))
	}
}

struct DrawSystem<'a>(&'a mut Context);

impl<'a> System<'a> for DrawSystem<'a> {
	type SystemData = (
		ReadStorage<'a, Location>,
		ReadStorage<'a, Size>,
		ReadStorage<'a, Color>,
		Read<'a, Camera>,
	);

	fn run(&mut self, (location, size, color, camera): Self::SystemData) {
		use specs::Join;
		let mut count = 0;

		for (location, size, color) in (&location, &size, &color).join() {
			if (location.0.x > camera.x && location.0.x < camera.x + camera.width)
				&& (location.0.y > camera.y && location.0.y < camera.y + camera.height)
			{
				let mesh = graphics::MeshBuilder::new()
					.circle(
						graphics::DrawMode::fill(),
						Point2::new(location.0.x - camera.x, location.0.y - camera.y),
						size.0,
						0.1,
						color.0,
					)
					.build(self.0)
					.unwrap();
				graphics::draw(self.0, &mesh, graphics::DrawParam::default()).unwrap();
				count += 1;
			}
		}

		let text = graphics::Text::new(format!("render count: {}", count));

		graphics::draw(
			self.0,
			&text,
			graphics::DrawParam::default().dest(Point2::new(10.0, 100.0)),
		)
		.unwrap();
	}
}

struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
	type SystemData = (
    Read<'a, DeltaTime>,
		ReadStorage<'a, Velocity>,
		WriteStorage<'a, Location>,
		Write<'a, Camera>,
	);

	fn run(&mut self, (delta, velocity, mut location, mut camera): Self::SystemData) {
    use specs::Join;

    let delta = delta.0;

		for (velocity, location) in (&velocity, &mut location).join() {
			location.0 += velocity.0 * delta;
			camera.x = location.0.x - camera.width / 2.0;
			camera.y = location.0.y - camera.height / 2.0;
		}
	}
}
