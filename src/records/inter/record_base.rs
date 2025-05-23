use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use crate::messages::inter::rr_types::RRTypes;

pub trait RecordBase: Debug {

    fn from_bytes(buf: &[u8], off: usize) -> Self where Self: Sized;

    fn to_bytes(&self, label_map: &mut HashMap<String, usize>, off: usize) -> Result<Vec<u8>, String>;

    fn get_type(&self) -> RRTypes;

    fn upcast(self) -> Box<dyn RecordBase>;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}
