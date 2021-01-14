cross_test::configure!();

#[cfg(test)]
mod test {
    use nash_ws::WebSocket;
    use nash_ws::Message;
    use libflate::gzip::Decoder;
    use std::io::Read;
    use cross_test::prelude::*;

    #[cross_test::test]
    async fn huobi() {
        let mut websocket = WebSocket::new("wss://api.huobi.pro/ws").await.expect("Couldn't connect to Huobi");
        let subscription_message = Message::Text(r#"
        {
          "sub": "market.ethbtc.kline.1min",
          "id": "id1"
        }
        "#.into());
        websocket.send(&subscription_message).await.expect("Couldn't send message.");
        while let Some(message) = websocket.next().await {
            let message = message.expect("Couldn't get message.");
            if let Message::Binary(binary) = message {
                let mut decoder = Decoder::new(&binary[..]).expect("Couldn't create Decoder.");
                let mut str = String::new();
                decoder.read_to_string(&mut str).expect("Couldn't read to String.");
                println!("{}", str);
            }
        }
        println!("Ended.")
    }
}
