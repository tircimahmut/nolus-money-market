use currencies::PaymentGroup;
use finance::price::dto::PriceDTO;

pub mod alarms;
pub mod config;
pub mod error;
pub mod feed;
pub mod feeders;
pub mod market_price;

#[cfg(test)]
mod tests;

type CurrencyGroup = PaymentGroup;
pub type SpotPrice = PriceDTO<PaymentGroup, PaymentGroup>;
