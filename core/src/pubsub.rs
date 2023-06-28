pub trait Observer<T>: Send {
    fn notify(&mut self, event: T);
}

pub trait Subject<T> {
    fn subscribe(&mut self, observer: impl Observer<T> + 'static);
    // fn unsubscribe(&mut self, observer: impl Observer<T> + 'static);
}
