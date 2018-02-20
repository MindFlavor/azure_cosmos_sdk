#![feature(conservative_impl_trait)]

#[macro_use]
extern crate azure_core_sdk;
extern crate base64;

#[macro_use]
extern crate hyper;
extern crate mime;

extern crate chrono;
extern crate crypto;

extern crate futures;
extern crate hyper_tls;
extern crate native_tls;
extern crate tokio_core;

#[macro_use]
extern crate log;
extern crate quick_error;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate url;

use std::fmt;
use azure_core_sdk::enumerations;
use azure_core_sdk::parsing::FromStringOptional;
use azure_core_sdk::errors::TraversingError;
use std::str::FromStr;

create_enum!(
    ConsistencyLevel,
    (Strong, "Strong"),
    (Bounded, "Bounded"),
    (Session, "Session"),
    (Eventual, "Eventual")
);

pub mod authorization_token;
pub mod collection;
pub mod database;
pub mod document;
pub mod partition_key;
pub mod get_document;
pub mod list_documents;
pub mod query;
pub mod query_document;
pub mod request_response;
pub mod client;
