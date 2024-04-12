use iced_core::Hasher;
use iced_futures::{
	self,
	futures::{channel::mpsc, stream::StreamExt},
	subscription,
};
use std::hash::Hash;

pub use grin_servers::ServerStats;

// TODO: Check https://github.com/iced-rs/iced/issues/336 for reference

#[derive(Clone, Debug)]
pub enum UIMessage {
	None,
	UpdateStatus(ServerStats),
}

pub enum State {
	Ready,
	Listening { receiver: mpsc::Receiver<UIMessage> },
	Finished,
}

pub fn subscriber<I: 'static + Hash + Copy + Send>(
	id: I,
) -> iced::Subscription<(I, UIMessage, Option<mpsc::Sender<UIMessage>>)> {
	iced::Subscription::from_recipe(NodeSubscriber { id })
}

pub struct NodeSubscriber<I> {
	id: I,
}

impl<T> iced_futures::subscription::Recipe for NodeSubscriber<T>
where
	T: 'static + Hash + Copy + Send,
{
	type Output = (T, UIMessage, Option<mpsc::Sender<UIMessage>>);

	fn hash(&self, state: &mut Hasher) {
		struct Marker;
		std::any::TypeId::of::<Marker>().hash(state);
		self.id.hash(state);
	}

	fn stream(
		self: Box<Self>,
		_input: subscription::EventStream,
	) -> futures::stream::BoxStream<'static, Self::Output> {
		let id = self.id;
		Box::pin(futures::stream::unfold(
			State::Ready,
			move |state| async move {
				match state {
					State::Ready => {
						let (sender, receiver) = mpsc::channel::<UIMessage>(0);
						Some((
							(id, UIMessage::None, Some(sender)),
							State::Listening { receiver },
						))
					}
					State::Listening { mut receiver } => match receiver.next().await {
						Some(msg) => Some(((id, msg, None), State::Listening { receiver })),
						_ => Some(((id, UIMessage::None, None), State::Listening { receiver })),
					},
					State::Finished => {
						// Don't let the stream die?
						let _: () = iced::futures::future::pending().await;
						None
					}
				}
			},
		))
	}
}
