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

fn draw_graph<F>(renderer: &Renderer, pos: Rect, color: Color, count: i16, func: F)
	where F: Fn(i16) -> i16
{
	let x = pos.x() as i16;
	let y = pos.y() as i16;
	let width = pos.width() as i16;
	let height = pos.height() as i16;

	let mut last = func(0);
	for i in 1..(count - 1)
	{
		let i = i as i16;
		let val = func(i);
		if val <= 0
		{
			continue;
		}

		renderer.thick_line(x + (i - 1) * width / count,
			y + height - last,
			x + i * width / count,
			y + height - val,
			2, color).unwrap();

		last = val;
	}

	renderer.thick_line(x, y, x, y + height, 2, color!(0, 0, 0)).unwrap();
	renderer.thick_line(x, y + height, x + width, y + height, 2, color!(0, 0, 0)).unwrap();
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

		draw_graph(&renderer, Rect::new(10, 10, 100, 100), color!(0, 0, 255), DATA_COUNT as i16, |i| {
			let next = (i as usize + pos + 1) % DATA_COUNT;
			let curr = (i as usize + pos) % DATA_COUNT;
			let total_diff = cpu_history[next].total() as i64 - cpu_history[curr].total() as i64;

			if total_diff == 0
			{
				return -1;
			}

			let idle_diff = cpu_history[next].idle() as i64 - cpu_history[curr].idle() as i64;
			return ((total_diff as f64 - idle_diff as f64) / total_diff as f64 * 100f64) as i16;
		});

		draw_graph(&renderer, Rect::new(120, 10, 100, 100), color!(255, 192, 0), DATA_COUNT as i16, |i| {
			let curr = (i as usize + pos) % DATA_COUNT;
			if(mem_history[curr].total == 0)
			{
				return -1;
			}

			return (100 - mem_history[curr].free * 100 / mem_history[curr].total) as i16;
		});
		draw_graph(&renderer, Rect::new(120, 10, 100, 100), color!(0, 255, 0), DATA_COUNT as i16, |i| {
			let curr = (i as usize + pos) % DATA_COUNT;
			if(mem_history[curr].total == 0)
			{
				return -1;
			}

		 	return (100 - mem_history[curr].available * 100 / mem_history[curr].total) as i16;
		});

		//TODO remote statistics etc.
		draw_graph(&renderer, Rect::new(10, 120, 100, 100), color!(0, 0, 0), 0, |i| -1);
		draw_graph(&renderer, Rect::new(120, 120, 100, 100), color!(0, 0, 0), 0, |i| -1);

		renderer.present();

		pos = (pos + 1) % DATA_COUNT;
		thread::sleep(time::Duration::from_millis(1000));
	}
}
