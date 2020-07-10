use crate::coinbase::model::Account;
use crate::coinbase::Coinbase;
use crate::Result;

impl Coinbase {
    pub async fn get_account(&self) -> Result<Vec<Account>> {
        self.transport.get_signed::<_, ()>("/accounts", None).await
    }
}
