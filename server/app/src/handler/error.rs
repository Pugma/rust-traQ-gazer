use openapi::apis::ErrorHandler;

use super::Handler;

#[async_trait::async_trait]
impl ErrorHandler for Handler {}
