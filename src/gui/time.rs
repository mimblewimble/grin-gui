use iced_futures::{self, subscription};

use iced_core::Hasher;
use std::hash::Hash;

pub fn every(duration: std::time::Duration) -> iced::Subscription<chrono::DateTime<chrono::Local>> {
	iced::Subscription::from_recipe(Every(duration))
}

struct Every(std::time::Duration);

impl iced_futures::subscription::Recipe for Every {
	type Output = chrono::DateTime<chrono::Local>;

	fn hash(&self, state: &mut Hasher) {
		use std::hash::Hash;

		std::any::TypeId::of::<Self>().hash(state);
		self.0.hash(state);
	}

	fn stream(
		self: Box<Self>,
		_input: subscription::EventStream,
	) -> futures::stream::BoxStream<'static, Self::Output> {
		use futures::stream::StreamExt;

		async_std::stream::interval(self.0)
			.map(|_| chrono::Local::now())
			.boxed()
	}
}
