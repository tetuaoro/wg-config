use std::collections::HashMap;

use ipnetwork::IpNetwork;

use crate::{WgConfError, WgPrivateKey};

/// Interface tag
pub const TAG: &'static str = "[Interface]";

// Fields
pub const PRIVATE_KEY: &'static str = "PrivateKey";
pub const ADDRESS: &'static str = "Address";
pub const LISTEN_PORT: &'static str = "ListenPort";
pub const POST_UP: &'static str = "PostUp";
pub const POST_DOWN: &'static str = "PostDown";

/// Represents WG [Interface] section
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WgInterface {
    pub(crate) private_key: WgPrivateKey,
    pub(crate) address: IpNetwork,
    pub(crate) listen_port: u16,
    pub(crate) post_up: String,
    pub(crate) post_down: String,
}

impl ToString for WgInterface {
    fn to_string(&self) -> String {
        format!(
            "{}
{} = {}
{} = {}
{} = {}
{} = {}
{} = {}
",
            TAG,
            PRIVATE_KEY,
            self.private_key.to_string(),
            ADDRESS,
            self.address.to_string(),
            LISTEN_PORT,
            self.listen_port,
            POST_UP,
            &self.post_up,
            POST_DOWN,
            &self.post_down
        )
    }
}

impl WgInterface {
    /// Creates new [`WgInterface`]
    ///
    /// Note, that WG address is address with mask (e.g. 10.0.0.1/8)
    pub fn new(
        private_key: WgPrivateKey,
        address: IpNetwork,
        listen_port: u16,
        post_up: String,
        post_down: String,
    ) -> Result<WgInterface, WgConfError> {
        if listen_port == 0 {
            return Err(WgConfError::ValidationFailed("port can't be 0".to_string()));
        }

        Ok(WgInterface {
            private_key,
            address,
            listen_port,
            post_up,
            post_down,
        })
    }

    /// Creates new [`WgInterface`] from raw String values
    pub fn from_raw_values(
        private_key: String,
        address: String,
        listen_port: String,
        post_up: String,
        post_down: String,
    ) -> Result<WgInterface, WgConfError> {
        let private_key: WgPrivateKey = private_key.parse()?;

        let address: IpNetwork = address.parse().map_err(|_| {
            WgConfError::ValidationFailed(format!(
                "address must be address with mask (e.g. 10.0.0.1/8)"
            ))
        })?;

        let listen_port: u16 = listen_port
            .parse()
            .map_err(|_| WgConfError::ValidationFailed("invalid port raw value".to_string()))?;

        if listen_port == 0 {
            return Err(WgConfError::ValidationFailed("port can't be 0".to_string()));
        }

        Ok(WgInterface {
            private_key,
            address,
            listen_port,
            post_up,
            post_down,
        })
    }

    pub(crate) fn from_raw_key_values(
        raw_key_values: HashMap<String, String>,
    ) -> Result<WgInterface, WgConfError> {
        let mut private_key = String::new();
        let mut address = String::new();
        let mut listen_port: String = String::new();
        let mut post_up = String::new();
        let mut post_down = String::new();

        for (k, v) in raw_key_values {
            match k {
                _ if k == PRIVATE_KEY => private_key = v,
                _ if k == ADDRESS => address = v,
                _ if k == LISTEN_PORT => listen_port = v,
                _ if k == POST_UP => post_up = v,
                _ if k == POST_DOWN => post_down = v,
                _ => continue,
            }
        }

        WgInterface::from_raw_values(private_key, address, listen_port, post_up, post_down)
    }

    // getters
    pub fn private_key(&self) -> &WgPrivateKey {
        &self.private_key
    }
    pub fn address(&self) -> &IpNetwork {
        &self.address
    }
    pub fn listen_port(&self) -> u16 {
        self.listen_port
    }
    pub fn post_up(&self) -> &str {
        &self.post_up
    }
    pub fn post_down(&self) -> &str {
        &self.post_down
    }
}
