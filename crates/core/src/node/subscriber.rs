use futures::channel::mpsc::UnboundedReceiver;
use iced_futures::{
    self,
    futures::{channel::mpsc, stream::StreamExt},
};
use std::hash::{Hash, Hasher};

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

pub fn subscriber<I: 'static + Hash + Copy + Send>(id: I) -> iced::Subscription<(I, UIMessage, Option<mpsc::Sender<UIMessage>>)> {
    iced::Subscription::from_recipe(NodeSubscriber{
        id
    })
}

pub struct NodeSubscriber<I> {
    id: I,
}

impl<H, I, T> iced_native::subscription::Recipe<H, I> for NodeSubscriber<T>
where
    T: 'static + Hash + Copy + Send,
    H: std::hash::Hasher,
{
    type Output = (T, UIMessage, Option<mpsc::Sender<UIMessage>>);

    fn hash(&self, state: &mut H) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);
        self.id.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
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
                        Some(msg) => {
                            debug!("Message {:?}", msg);
                            Some((
                                (id, msg, None),
                                State::Listening { receiver },
                            ))
                        }
                        _ => Some((
                            (id, UIMessage::None, None),
                            State::Listening { receiver },
                        )),
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
