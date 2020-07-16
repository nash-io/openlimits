use crate::model::{Success, UserDataStream};
use crate::Binance;

use shared::Result;

static USER_DATA_STREAM: &str = "/api/v3/userDataStream";

impl Binance {
    // User Stream
    pub async fn user_stream_start(&self) -> Result<UserDataStream> {
        let user_data_stream = self.transport.post::<_, ()>(USER_DATA_STREAM, None).await?;
        Ok(user_data_stream)
    }

    // Current open orders on a symbol
    pub async fn user_stream_keep_alive(&self, listen_key: &str) -> Result<Success> {
        let success = self
            .transport
            .put(
                USER_DATA_STREAM,
                Some(&vec![("listen_key", listen_key.to_string())]),
            )
            .await?;
        Ok(success)
    }

    pub async fn user_stream_close(&self, listen_key: &str) -> Result<Success> {
        let success = self
            .transport
            .delete(
                USER_DATA_STREAM,
                Some(&vec![("listen_key", listen_key.to_string())]),
            )
            .await?;
        Ok(success)
    }
}
