use crate::common::store_type::StoreType;

pub trait ObjectStore {
    fn save(&self) -> anyhow::Result<StoreType>;
}
