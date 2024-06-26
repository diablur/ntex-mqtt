use std::{fmt, marker::PhantomData, task::Context, task::Poll};

use ntex::service::{Service, ServiceFactory};
use ntex::util::Ready;

use super::control::{ControlMessage, ControlResult};
use super::publish::{PublishMessage, PublishResult};
use super::Session;

/// Default publish service
pub struct DefaultPublishService<St, Err> {
    _t: PhantomData<(St, Err)>,
}

impl<St, Err> Default for DefaultPublishService<St, Err> {
    fn default() -> Self {
        Self { _t: PhantomData }
    }
}

impl<St, Err> ServiceFactory for DefaultPublishService<St, Err> {
    type Config = Session<St>;
    type Request = PublishMessage;
    type Response = PublishResult;
    type Error = Err;
    type Service = DefaultPublishService<St, Err>;
    type InitError = Err;
    type Future = Ready<Self::Service, Self::InitError>;

    fn new_service(&self, _: Session<St>) -> Self::Future {
        Ready::Ok(DefaultPublishService { _t: PhantomData })
    }
}

impl<St, Err> Service for DefaultPublishService<St, Err> {
    type Request = PublishMessage;
    type Response = PublishResult;
    type Error = Err;
    type Future = Ready<Self::Response, Self::Error>;

    fn poll_ready(&self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&self, req: PublishMessage) -> Self::Future {
        log::warn!("Publish service is disabled");
        Ready::Ok(req.ack())
    }
}

/// Default control service
pub struct DefaultControlService<S, E>(PhantomData<(S, E)>);

impl<S, E: fmt::Debug> Default for DefaultControlService<S, E> {
    fn default() -> Self {
        DefaultControlService(PhantomData)
    }
}

impl<S, E: fmt::Debug> ServiceFactory for DefaultControlService<S, E> {
    type Config = Session<S>;
    type Request = ControlMessage<E>;
    type Response = ControlResult;
    type Error = E;
    type InitError = E;
    type Service = DefaultControlService<S, E>;
    type Future = Ready<Self::Service, Self::InitError>;

    fn new_service(&self, _: Session<S>) -> Self::Future {
        Ready::Ok(DefaultControlService(PhantomData))
    }
}

impl<S, E: fmt::Debug> Service for DefaultControlService<S, E> {
    type Request = ControlMessage<E>;
    type Response = ControlResult;
    type Error = E;
    type Future = Ready<Self::Response, Self::Error>;

    #[inline]
    fn poll_ready(&self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    #[inline]
    fn call(&self, pkt: Self::Request) -> Self::Future {
        match pkt {
            ControlMessage::Ping(pkt) => Ready::Ok(pkt.ack()),
            ControlMessage::Disconnect(pkt) => Ready::Ok(pkt.ack()),
            _ => {
                log::warn!("MQTT Control service is not configured, pkt: {:?}", pkt);
                Ready::Ok(pkt.disconnect_with(super::codec::Disconnect::new(
                    super::codec::DisconnectReasonCode::UnspecifiedError,
                )))
            }
        }
    }
}
