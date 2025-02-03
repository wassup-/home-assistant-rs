impl Client {
    /// Reboots the host.
    pub async fn post_host_reboot(&self) -> Result<(), ClientError> {
        let res = self.client.post("/host/reboot", ()).await?;
        Ok(res)
    }
}

use crate::supervisor::{Client, ClientError};
