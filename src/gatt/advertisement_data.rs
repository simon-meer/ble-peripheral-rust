use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct AdvertisementData {
    pub uuids: Vec<Uuid>,
    pub manufacturer_data: BTreeMap<u16, Vec<u8>>,
    pub service_data: BTreeMap<Uuid, Vec<u8>>,
}

impl Default for AdvertisementData {
    fn default() -> Self {
        AdvertisementData {
            uuids: vec![],
            manufacturer_data: BTreeMap::new(),
            service_data: BTreeMap::new(),
        }
    }
}
