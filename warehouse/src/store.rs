use super::package::{MQMessage, PackageInfo};
use bincode;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use chrono::Utc;
use logging::{log_info, log_error};

#[derive(Clone)]
pub struct Warehouse {
    packages: Arc<Mutex<VecDeque<PackageInfo>>>,
}

impl Warehouse {
    pub fn new() -> Self {
        Warehouse {
            packages: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn store_package(&self, message: MQMessage) {
        let id = Uuid::new_v4().to_string();
        let received = Utc::now().to_rfc3339();
        let size = bincode::serialized_size(&message).unwrap_or(0) as usize;

        let package_info = PackageInfo {
            id: id.clone(),
            received: received.clone(),
            size,
            message,
        };

        let mut packages = self.packages.lock().unwrap();
        packages.push_back(package_info);
        log_info(&format!("Stored package with ID {} at {}", id, received));
    }

    pub fn getPackagetByID(&self, id: &str) -> Option<PackageInfo> {
        let packages = self.packages.lock().unwrap();
        packages.iter().find(|&p| p.id == id).cloned()
    }

    pub fn fetch_package(&self, queue: &str) -> Option<PackageInfo> {
        let mut packages = self.packages.lock().unwrap();
        if let Some(position) = packages.iter().position(|p| match &p.message {
            MQMessage::SendMessage { queue: q, .. } => q == queue,
            _ => false,
        }) {
            let package = packages.remove(position).unwrap();
            log_info(&format!("Fetched package with ID {} from queue {}", package.id, queue));
            Some(package)
        } else {
            log_error(&format!("No package found in queue {}", queue));
            None
        }
    }

    pub fn get_next_package_from_queue(&self, queue: &str) -> Option<PackageInfo> {
        let mut packages = self.packages.lock().unwrap();
        if let Some(position) = packages.iter().position(|p| match &p.message {
            MQMessage::SendMessage { queue: q, .. } => q == queue,
            _ => false,
        }) {
            let package = packages.remove(position).unwrap();
            log_info(&format!("Fetched next package with ID {} from queue {}", package.id, queue));
            Some(package)
        } else {
            log_error(&format!("No packages available in queue {}", queue));
            None
        }
    }

    pub fn get_all_packages(&self) -> Vec<PackageInfo> {
        let packages: Vec<PackageInfo> = self.packages.lock().unwrap().clone().into();
        packages
    }

    pub fn get_package_size(&self, sender: &str) -> usize {
        let packages = self.packages.lock().unwrap();
        let total_size: usize = packages.iter()
            .filter(|p| match &p.message {
                MQMessage::SendMessage { message, .. } => message.sender.sender == sender,
                _ => false,
            })
            .map(|p| p.size)
            .sum();
        log_info(&format!("Total size of packages for sender {}: {} bytes", sender, total_size));
        total_size
    }
}
