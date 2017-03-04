use std::*;

extern crate sdl2;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Renderer;
use sdl2::render::TextureQuery;
use sdl2::gfx::primitives::DrawRenderer;

mod cpu;
mod mem;

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 600;
const DATA_COUNT: usize = 50;
const FONT: &'static str = "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf";

macro_rules! color
{
	($r:expr, $g:expr, $b:expr) => { Color::RGBA($r, $g, $b, 255); }
}

fn main()
{
	let mut pos = 0;
	let mut cpu_history: [cpu::Usage; DATA_COUNT] = [Default::default(); DATA_COUNT];
	let mut mem_history: [mem::Usage; DATA_COUNT] = [Default::default(); DATA_COUNT];

	let sdl = sdl2::init().unwrap();
	let video = sdl.video().unwrap();
	let ttf = sdl2::ttf::init().unwrap();

	let window = video.window("Status Screen", SCREEN_WIDTH, SCREEN_HEIGHT)
		.position_centered()
		//.borderless()
		.build()
		.unwrap();

	let mut renderer = window.renderer().build().unwrap();
	let font = ttf.load_font(FONT, 12).unwrap();

	/*let surface = font.render("Hello Rust!").blended(color!(255, 0, 0)).unwrap();
	let mut texture = renderer.create_texture_from_surface(&surface).unwrap();
	let TextureQuery { width, height, .. } = texture.query();

	let layout = Rect::new(0, 0, width, height);
	renderer.copy(&mut texture, None, Some(layout)).unwrap();*/

	loop
	{
		cpu_history[pos] = cpu::read_usage();
		mem_history[pos] = mem::read_usage();

		renderer.set_draw_color(color!(255, 255, 255));
		renderer.clear();

		renderer.thick_line(10, 10, 10, 110, 2, color!(0, 0, 0)).unwrap();
		renderer.thick_line(10, 110, 110, 110, 2, color!(0, 0, 0)).unwrap();
		renderer.thick_line(120, 10, 120, 110, 2, color!(0, 0, 0)).unwrap();
		renderer.thick_line(120, 110, 220, 110, 2, color!(0, 0, 0)).unwrap();

		renderer.thick_line(10, 120, 10, 220, 2, color!(0, 0, 0)).unwrap();
		renderer.thick_line(10, 220, 110, 220, 2, color!(0, 0, 0)).unwrap();
		renderer.thick_line(120, 120, 120, 220, 2, color!(0, 0, 0)).unwrap();
		renderer.thick_line(120, 220, 220, 220, 2, color!(0, 0, 0)).unwrap();

		let mut cpu_last_usage: i64 = -1;
		let mut mem_last_hardusage: i64 = -1;
		let mut mem_last_usage: i64 = -1;
		for i in 1 .. DATA_COUNT
		{
			let curr = (pos + i) % DATA_COUNT;

			{
				let surface = font.render("CPU").blended(color!(0, 0, 0)).unwrap();
				let mut texture = renderer.create_texture_from_surface(&surface).unwrap();
				let TextureQuery { width, height, .. } = texture.query();

				let pos1 = Rect::new(13, 10, width, height);
				renderer.copy(&mut texture, None, Some(pos1)).unwrap();

				let pos2 = Rect::new(13, 120, width, height);
				renderer.copy(&mut texture, None, Some(pos2)).unwrap();
			}
			{
				let surface = font.render("MEM").blended(color!(0, 0, 0)).unwrap();
				let mut texture = renderer.create_texture_from_surface(&surface).unwrap();
				let TextureQuery { width, height, .. } = texture.query();

				let pos1 = Rect::new(123, 10, width, height);
				renderer.copy(&mut texture, None, Some(pos1)).unwrap();

				let pos2 = Rect::new(123, 120, width, height);
				renderer.copy(&mut texture, None, Some(pos2)).unwrap();
			}

			{
				let next = (curr + 1) % DATA_COUNT;
				let total_diff = cpu_history[next].total() as i64 - cpu_history[curr].total() as i64;
				let idle_diff = cpu_history[next].idle() as i64 - cpu_history[curr].idle() as i64;

				if total_diff != 0
				{
					let usage = ((total_diff as f64 - idle_diff as f64) / total_diff as f64 * 100f64) as i64;

					if cpu_last_usage != -1
					{
						renderer.thick_line((11 + (i - 1) * 100 / DATA_COUNT) as i16,
							(110 - cpu_last_usage) as i16,
							(11 + i * 100 / DATA_COUNT) as i16,
							(110 - usage) as i16,
							2, color!(0, 0, 255)).unwrap();
					}
					cpu_last_usage = usage;
				}
			}

			{
				if mem_history[curr].total != 0
				{
					let hardused: i64 = (100 - mem_history[curr].available * 100 / mem_history[curr].total) as i64;
					let used: i64 = (100 - mem_history[curr].free * 100 / mem_history[curr].total) as i64;

					if mem_last_usage != -1
					{
						renderer.thick_line((121 + (i - 1) * 100 / DATA_COUNT) as i16,
							(110 - mem_last_hardusage) as i16,
							(121 + i * 100 / DATA_COUNT) as i16,
							(110 - hardused) as i16,
							2, color!(0, 255, 0)).unwrap();

						renderer.thick_line((121 + (i - 1) * 100 / DATA_COUNT) as i16,
							(110 - mem_last_usage) as i16,
							(121 + i * 100 / DATA_COUNT) as i16,
							(110 - used) as i16,
							2, color!(255, 192, 0)).unwrap();
					}
					mem_last_hardusage = hardused;
					mem_last_usage = used;
				}
			}
		}

		renderer.present();

		pos = (pos + 1) % DATA_COUNT;
		thread::sleep(time::Duration::from_millis(1000));
	}
}
