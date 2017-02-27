use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Default, Copy, Clone)]
pub struct Usage
{
	pub total: u64,
	pub free: u64,
	pub available: u64,
	pub buffers: u64,
	pub cache: u64,
}

pub fn read_usage() -> Usage
{
	let file = BufReader::new(File::open("/proc/meminfo").unwrap());
	let mut usage: Usage = Default::default();

	for line in file.lines()
	{
		let pls = line.unwrap();
		let mut parts = pls.split_whitespace();

		match parts.next().unwrap()
		{
			"MemTotal:" 	=> { usage.total = parts.next().unwrap().parse().unwrap(); }
			"MemFree:" 		=> { usage.free = parts.next().unwrap().parse().unwrap(); }
			"MemAvailable:"	=> { usage.available = parts.next().unwrap().parse().unwrap(); }
			"Buffers:"		=> { usage.buffers = parts.next().unwrap().parse().unwrap(); }
			"Cached:"		=> { usage.cache = parts.next().unwrap().parse().unwrap(); }
			_				=> {  }
		}
	}

	return usage;
}
