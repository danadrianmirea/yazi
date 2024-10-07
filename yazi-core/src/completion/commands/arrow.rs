use yazi_shared::{event::{Cmd, Data}, render};

use crate::completion::Completion;

pub struct Opt {
	step: isize,
}

impl From<Cmd> for Opt {
	fn from(c: Cmd) -> Self { Self { step: c.first().and_then(Data::as_isize).unwrap_or(0) } }
}

impl Completion {
	#[yazi_macro::command]
	pub fn arrow(&mut self, opt: Opt) {
		if opt.step > 0 {
			self.next(opt.step as usize);
		} else {
			self.prev(opt.step.unsigned_abs());
		}
	}

	fn next(&mut self, step: usize) {
		let len = self.cands.len();
		if len == 0 {
			return;
		}

		let old = self.cursor;
		self.cursor = (self.cursor + step).min(len - 1);

		let limit = self.limit();
		if self.cursor >= len.min(self.offset + limit) {
			self.offset = len.saturating_sub(limit).min(self.offset + self.cursor - old);
		}

		render!(old != self.cursor);
	}

	fn prev(&mut self, step: usize) {
		let old = self.cursor;
		self.cursor = self.cursor.saturating_sub(step);

		if self.cursor < self.offset {
			self.offset = self.offset.saturating_sub(old - self.cursor);
		}

		render!(old != self.cursor);
	}
}
