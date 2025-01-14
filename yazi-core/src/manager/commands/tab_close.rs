use yazi_shared::event::Exec;

use crate::manager::Tabs;

pub struct Opt {
	idx: usize,
}

impl From<&Exec> for Opt {
	fn from(e: &Exec) -> Self {
		Self { idx: e.args.first().and_then(|i| i.parse().ok()).unwrap_or(0) }
	}
}

impl From<usize> for Opt {
	fn from(idx: usize) -> Self { Self { idx } }
}

impl Tabs {
	pub fn close(&mut self, opt: impl Into<Opt>) -> bool {
		let opt = opt.into() as Opt;

		let len = self.items.len();
		if len < 2 || opt.idx >= len {
			return false;
		}

		self.items.remove(opt.idx);
		if opt.idx <= self.idx {
			self.set_idx(self.absolute(1));
		}

		true
	}
}
