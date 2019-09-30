use actix_service::Service;
use actix_testing::{block_on, TestServer};
use bytes::Bytes;
use futures::future::ok;
use futures::Future;
use string::TryFrom;

use actix_mqtt::{client, Connect, ConnectAck, MqttServer, Publish};

struct Session;

fn connect<Io>(packet: Connect<Io>) -> impl Future<Item = ConnectAck<Io, Session>, Error = ()> {
    println!("CONNECT: {:?}", packet);
    ok(packet.ack(Session, false))
}

#[test]
fn test_simple() -> std::io::Result<()> {
    std::env::set_var(
        "RUST_LOG",
        "actix_codec=info,actix_server=trace,actix_connector=trace",
    );
    env_logger::init();

    let srv = TestServer::with(|| MqttServer::new(connect).finish(|_t| Ok(())));

    struct ClientSession;

    let mut client =
        client::Client::new(string::String::try_from(Bytes::from_static(b"user")).unwrap())
            .state(|ack: client::ConnectAck<_>| {
                ack.sink().publish_qos0(
                    string::String::try_from(Bytes::from_static(b"#")).unwrap(),
                    Bytes::new(),
                    false,
                );
                ack.sink().close();
                Ok(ack.state(ClientSession))
            })
            .finish(|_t: Publish<_>| {
                // t.sink().close();
                Ok(())
            });

    let conn = block_on(
        actix_connect::default_connector()
            .call(actix_connect::Connect::with(String::new(), srv.addr())),
    )
    .unwrap();

    block_on(client.call(conn.into_parts().0)).unwrap();

    Ok(())
}
