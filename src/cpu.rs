use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

#[derive(Default, Copy, Clone)]
pub struct Usage
{
	pub user : u64,
	pub nice : u64,
	pub system : u64,
	pub halt : u64,
	pub iowait : u64,
	pub irq : u64,
	pub softirq : u64
}

impl Usage
{
	pub fn total(&self) -> u64
	{
		self.user + self.nice + self.system + self.halt + self.iowait + self.irq + self.softirq
	}
	pub fn idle(&self) -> u64
	{
		self.halt + self.iowait
	}
}

pub fn read_usage() -> Usage
{
	let file = BufReader::new(File::open("/proc/stat").unwrap());

	for line in file.lines()
	{
		let l = line.unwrap();
		if l.starts_with("cpu ")
		{
			let mut parts = l.split_whitespace();
			assert!(parts.next().unwrap() == "cpu");

			return Usage {
				user: parts.next().unwrap().parse().unwrap(),
				nice: parts.next().unwrap().parse().unwrap(),
				system: parts.next().unwrap().parse().unwrap(),
				halt: parts.next().unwrap().parse().unwrap(),
				iowait: parts.next().unwrap().parse().unwrap(),
				irq: parts.next().unwrap().parse().unwrap(),
				softirq: parts.next().unwrap().parse().unwrap(),
			};
		}
	}

	panic!();
}
