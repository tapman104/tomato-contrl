//! mDNS Discovery module for remctrl.
use crate::transport::tcp::DEFAULT_PORT;
use anyhow::{Context, Result};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;

/// Dropping this stops the mDNS advertisement.
pub struct MdnsHandle {
    daemon: ServiceDaemon,
    service_type: String,
    instance_name: String,
}

impl Drop for MdnsHandle {
    fn drop(&mut self) {
        let fullname = format!("{}.{}", self.instance_name, self.service_type);
        // Unregister the service from the daemon
        let _ = self.daemon.unregister(&fullname);
    }
}

/// Advertise this device as a remctrl server on the LAN.
/// The service is broadcast until the returned MdnsHandle is dropped.
///
/// Service type:  "_remctrl._tcp.local."
/// Service name:  "RemoteCtrl-{hostname}"
/// Port:          remctrl::transport::tcp::DEFAULT_PORT (9847)
///
/// TXT record includes:
///   "version" = "1"
pub fn advertise() -> Result<MdnsHandle> {
    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().into_owned())
        .unwrap_or_else(|_| "remctrl-pc".to_string());

    let service_type = "_remctrl._tcp.local.";
    let instance_name = format!("RemoteCtrl-{}", hostname);
    let host_name = format!("{}.local.", hostname);

    let daemon = ServiceDaemon::new().context("Failed to create mDNS daemon")?;

    let mut properties = HashMap::new();
    properties.insert("version".to_string(), "1".to_string());

    let my_ip = "";
    
    let service_info = ServiceInfo::new(
        service_type,
        &instance_name,
        &host_name,
        my_ip,
        DEFAULT_PORT,
        Some(properties),
    ).context("Failed to create mDNS ServiceInfo")?;

    daemon.register(service_info).context("Failed to register mDNS service")?;

    Ok(MdnsHandle {
        daemon,
        service_type: service_type.to_string(),
        instance_name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advertise_does_not_panic() {
        let handle = advertise();
        assert!(handle.is_ok());
        drop(handle);
    }
}
