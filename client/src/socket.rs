use core::messages::{ClientMessage, GenericClientMessage};

pub fn send_message<T: ClientMessage>(message: T) -> T::ServerResponse {
    let generic_message: GenericClientMessage = message.into();
    todo!("Send message");
    //
}
