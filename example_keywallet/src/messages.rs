//! Data types needed for communication between service and client

use rustbus::wire::marshal::traits::ObjectPath;
use rustbus_derive::Marshal;
use rustbus_derive::Signature;
use rustbus_derive::Unmarshal;

#[derive(Marshal, Unmarshal, Signature, Clone)]
pub struct Secret {
    pub session: ObjectPath<String>,
    pub params: Vec<u8>,
    pub value: Vec<u8>,
    pub content_type: String,
}