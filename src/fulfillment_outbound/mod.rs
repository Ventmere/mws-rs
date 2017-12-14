//! Fulfillment Outbound Shipment API - Version 2010-10-01
//!
//! [Documentation](https://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_Overview.html)

use chrono::{DateTime, Utc};
use client::{Client, Method, Response};
mod types;
pub use self::types::*;
use xmlhelper::decode;
use super::types::ToIso8601;

error_chain! {
  links {
    Client(super::client::Error, super::client::ErrorKind);
    Decode(decode::Error, decode::ErrorKind);
  }
}

static PATH: &'static str = "/FulfillmentOutboundShipment/2010-10-01";
static VERSION: &'static str = "2010-10-01";

#[derive(Debug, Default)]
pub struct ListAllFulfillmentOrdersResponse {
  pub request_id: String,
  pub fulfillment_orders: Vec<FulfillmentOrder>,
  pub next_token: Option<String>,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for ListAllFulfillmentOrdersResponse {
  fn from_xml(s: &mut S) -> decode::Result<ListAllFulfillmentOrdersResponse> {
    use self::decode::{start_document, element, fold_elements, characters};
    start_document(s)?;
    element(
      s,
      vec![
        "ListAllFulfillmentOrdersResponse",
        "ListAllFulfillmentOrdersByNextTokenResponse",
      ],
      |s| {
        fold_elements(
          s,
          ListAllFulfillmentOrdersResponse::default(),
          |s, response| match s.local_name() {
            "ListAllFulfillmentOrdersResult" |
            "ListAllFulfillmentOrdersByNextTokenResult" => {
              fold_elements(s, (), |s, _| {
                match s.local_name() {
                  "FulfillmentOrders" => {
                    response.fulfillment_orders = fold_elements(s, vec![], |s, list| {
                      list.push(FulfillmentOrder::from_xml(s)?);
                      Ok(())
                    })?;
                  }
                  "NextToken" => {
                    response.next_token = Some(characters(s)?);
                  }
                  _ => {}
                }
                Ok(())
              })
            }
            "ResponseMetadata" => {
              response.request_id = element(s, "RequestId", |s| characters(s))?;
              Ok(())
            }
            _ => Ok(()),
          },
        )
      },
    )
  }
}

/// Returns a list of fulfillment orders fulfilled after (or at) a specified date.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_US/orders-2013-09-01/Orders_ListOrders.html)
#[allow(non_snake_case)]
pub fn ListAllFulfillmentOrders(
  client: &Client,
  query_start_date_time: DateTime<Utc>,
) -> Result<Response<ListAllFulfillmentOrdersResponse>> {
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListAllFulfillmentOrders",
      vec![
        (
          "QueryStartDateTime".to_string(),
          query_start_date_time.to_iso8601()
        ),
      ],
    )
    .map_err(|err| err.into())
}

/// Returns the next page of fulfillment orders using the NextToken parameter.
///
/// [Documentation](https://docs.developer.amazonservices.com/en_US/fba_outbound/FBAOutbound_ListAllFulfillmentOrdersByNextToken.html)
#[allow(non_snake_case)]
pub fn ListAllFulfillmentOrdersByNextToken(
  client: &Client,
  next_token: String,
) -> Result<Response<ListAllFulfillmentOrdersResponse>> {
  let params = vec![("NextToken".to_string(), next_token)];
  client
    .request_xml(
      Method::Post,
      PATH,
      VERSION,
      "ListAllFulfillmentOrdersByNextToken",
      params,
    )
    .map_err(|err| err.into())
}

#[derive(Debug, Default)]
#[allow(non_snake_case)]
pub struct GetFulfillmentOrderResponse {
  pub fulfillment_shipments: Vec<FulfillmentShipment>,
  pub return_item_list: Vec<ReturnItemList>,
  pub return_authorization_list: Vec<ReturnAuthorizationList>,
  pub fulfillment_order: FulfillmentOrder,
  pub fulfillment_order_items: Vec<FulfillmentOrderItem>,
  pub request_id: String,
}

impl<S: decode::XmlEventStream> decode::FromXMLStream<S> for GetFulfillmentOrderResponse {
  fn from_xml(s: &mut S) -> decode::Result<GetFulfillmentOrderResponse> {
    use self::decode::{start_document, element, fold_elements, characters};
    start_document(s)?;
    element(s, "GetFulfillmentOrderResponse", |s| {
      fold_elements(s, GetFulfillmentOrderResponse::default(), |s, response| {
        match s.local_name() {
          "GetFulfillmentOrderResult" => {
            fold_elements(s, (), |s, _| {
              match s.local_name() {
                "FulfillmentShipment" => {
                  response.fulfillment_shipments = fold_elements(s, vec![], |s, list| {
                    list.push(FulfillmentShipment::from_xml(s)?);
                    Ok(())
                  })?;
                }
                "FulfillmentOrder" => response.fulfillment_order = FulfillmentOrder::from_xml(s)?,
                "FulfillmentOrderItem" => {
                  response.fulfillment_order_items = fold_elements(s, vec![], |s, list| {
                    list.push(FulfillmentOrderItem::from_xml(s)?);
                    Ok(())
                  })?
                }
                _ => {}
              }
              Ok(())
            })
          }
          "ResponseMetadata" => {
            response.request_id = element(s, "RequestId", |s| characters(s))?;
            Ok(())
          }
          _ => Ok(()),
        }
      })
    })
  }
}

/// Returns a fulfillment order based on a specified SellerFulfillmentOrderId.
///
/// [Documentation](http://docs.developer.amazonservices.com/en_CA/fba_outbound/FBAOutbound_GetFulfillmentOrder.html)
#[allow(non_snake_case)]
pub fn GetFulfillmentOrder(
  client: &Client,
  seller_fulfillment_order_id: String,
) -> Result<Response<GetFulfillmentOrderResponse>> {
  let params = vec![
    (
      "SellerFulfillmentOrderId".to_string(),
      seller_fulfillment_order_id
    ),
  ];
  client
    .request_xml(Method::Post, PATH, VERSION, "GetFulfillmentOrder", params)
    .map_err(|err| err.into())
}