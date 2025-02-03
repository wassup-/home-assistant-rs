impl Client {
    /// Returns information about the supervisor.
    pub async fn get_supervisor_info(&self) -> Result<entities::SupervisorInfo, ClientError> {
        let info = self.client.get("/supervisor/info", ()).await?;
        Ok(info)
    }
}

use crate::supervisor::{entities, Client, ClientError};
