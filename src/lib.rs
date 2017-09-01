//! Client library for Amazon Marketplace Web Service (Amazon MWS)
//!

extern crate url;
extern crate xml;
extern crate chrono;
extern crate crypto;
extern crate reqwest;
extern crate base64;
#[macro_use] extern crate error_chain;
extern crate csv;

#[cfg(test)] extern crate dotenv;

#[macro_use] pub mod xmlhelper;
mod types;
#[macro_use] mod macros;
#[macro_use] pub mod tdff;
mod sign;
pub mod client;


// pub mod products;
pub mod orders;
pub mod reports;
pub mod fulfillment_inventory;
pub mod fulfillment_inbound_shipment;
pub mod feeds;