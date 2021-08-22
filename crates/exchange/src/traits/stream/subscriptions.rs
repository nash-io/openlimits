use crate::model::websocket::Subscription;

pub struct Subscriptions<T: From<Subscription>> {
    inner: Vec<T>,
}

impl<T: From<Subscription>> Subscriptions<T> {
    pub fn as_slice(&self) -> &[T] {
        &self.inner[..]
    }
}

impl<T: From<Subscription>> IntoIterator for Subscriptions<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<T: From<Subscription>, U: Into<T> + Clone> From<&[U]> for Subscriptions<T> {
    fn from(s: &[U]) -> Self {
        let v = s.iter().cloned().map(U::into).collect::<Vec<_>>();

        Subscriptions { inner: v }
    }
}