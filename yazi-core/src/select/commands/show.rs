use anyhow::{anyhow, Result};
use tokio::sync::oneshot;
use yazi_config::popup::SelectCfg;
use yazi_shared::{emit, event::Exec, term::Term, Layer};

use crate::select::Select;

pub struct Opt {
	cfg: SelectCfg,
	tx:  oneshot::Sender<Result<usize>>,
}

impl TryFrom<&Exec> for Opt {
	type Error = anyhow::Error;

	fn try_from(e: &Exec) -> Result<Self, Self::Error> {
		e.take_data().ok_or_else(|| anyhow!("invalid data"))
	}
}

impl Select {
	pub async fn _show(cfg: SelectCfg) -> Result<usize> {
		let (tx, rx) = oneshot::channel();
		emit!(Call(Exec::call("show", vec![]).with_data(Opt { cfg, tx }).vec(), Layer::Select));
		rx.await.unwrap_or_else(|_| Term::goodbye(|| false))
	}

	pub fn show(&mut self, opt: impl TryInto<Opt>) -> bool {
		let Ok(opt) = opt.try_into() else {
			return false;
		};

		self.close(false);
		self.title = opt.cfg.title;
		self.items = opt.cfg.items;
		self.position = opt.cfg.position;

		self.callback = Some(opt.tx);
		self.visible = true;
		true
	}
}
