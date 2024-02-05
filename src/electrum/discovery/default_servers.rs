use crate::chain::Network;
#[allow(unused_imports)]
use crate::electrum::discovery::{DiscoveryManager, Service};

#[allow(unused_variables)]
pub fn add_default_servers(discovery: &DiscoveryManager, network: Network) {
    match network {
        #[cfg(not(feature = "liquid"))]
        Network::Bitcoin => {
            discovery
                .add_default_server(
                    "bitexplorer.io".into(),
                    vec![Service::Tcp(50001), Service::Ssl(50002)],
                )
                .ok();





        _ => (),
    }
}
