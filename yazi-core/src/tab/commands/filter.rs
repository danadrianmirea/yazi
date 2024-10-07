use std::time::Duration;

use tokio::pin;
use tokio_stream::{StreamExt, wrappers::UnboundedReceiverStream};
use yazi_config::popup::InputCfg;
use yazi_fs::FilterCase;
use yazi_proxy::InputProxy;
use yazi_shared::{Debounce, InputError, Layer, emit, event::Cmd};

use crate::tab::Tab;

#[derive(Default)]
pub struct Opt {
	pub query: String,
	pub case:  FilterCase,
	pub done:  bool,
}

impl From<Cmd> for Opt {
	fn from(mut c: Cmd) -> Self {
		Self {
			query: c.take_first_str().unwrap_or_default(),
			case:  FilterCase::from(&c),
			done:  c.bool("done"),
		}
	}
}

impl Tab {
	#[yazi_macro::command]
	pub fn filter(&mut self, opt: Opt) {
		tokio::spawn(async move {
			let rx = InputProxy::show(InputCfg::filter());

			let rx = Debounce::new(UnboundedReceiverStream::new(rx), Duration::from_millis(50));
			pin!(rx);

			while let Some(result) = rx.next().await {
				let done = result.is_ok();
				let (Ok(s) | Err(InputError::Typed(s))) = result else { continue };

				emit!(Call(
					Cmd::args("filter_do", &[s])
						.with_bool("smart", opt.case == FilterCase::Smart)
						.with_bool("insensitive", opt.case == FilterCase::Insensitive)
						.with_bool("done", done),
					Layer::Manager
				));
			}
		});
	}
}
