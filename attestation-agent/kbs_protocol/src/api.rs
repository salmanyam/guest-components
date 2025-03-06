// Copyright (c) 2023 Alibaba Cloud
//
// SPDX-License-Identifier: Apache-2.0
//

use crate::Result;
use async_trait::async_trait;
pub use resource_uri::ResourceUri;

#[async_trait]
pub trait KbsClientCapabilities {
    async fn get_resource(&mut self, resource_uri: ResourceUri) -> Result<Vec<u8>>;
    async fn get_plugin_resource(&mut self, plugin_name: String, resource_path: String) -> Result<Vec<u8>>;
}
