use futures::channel::mpsc::UnboundedReceiver;
use grin_gui_core::node::{ServerStats, UIMessage};
use iced::futures;

pub fn node_sub(id: &str, receiver: UnboundedReceiver<UIMessage>) -> iced::Subscription<UIMessage> {
    iced::Subscription::from_recipe(NodeSubscriber(id.to_owned(), receiver))
}

struct NodeSubscriber(String, UnboundedReceiver<UIMessage>);

impl<H, I> iced_native::subscription::Recipe<H, I> for NodeSubscriber
where
    H: std::hash::Hasher,
{
    type Output = UIMessage;

    fn hash(&self, state: &mut H) {
        use std::hash::Hash;

        std::any::TypeId::of::<Self>().hash(state);
        self.0.hash(state);
    }

    fn stream(
        self: Box<Self>,
        _input: futures::stream::BoxStream<'static, I>,
    ) -> futures::stream::BoxStream<'static, Self::Output> {
        use futures::stream::StreamExt;
        let mut rx = self.1;

        Box::pin(futures::stream::unfold(
            UIMessage::Ready,
            move |_| async move {
                match rx.try_next() {
                    Ok(Some(m)) => Some((m, UIMessage::Ready)),
                    Ok(None) => None,
                    Err(_) => None
                }
            },
        ))
    }
}
