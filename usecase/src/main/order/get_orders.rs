use derive_new::new;
use thiserror::Error;

use domain::main::order::value_objects::shop_order_id::ShopOrderId;

use crate::main::order::dto::order_details::OrderDetails;

pub trait GetOrders {
    fn execute(
        &self,
        start_id: ShopOrderId,
        limit: i32,
    ) -> Result<Vec<OrderDetails>, GetOrdersUseCaseError>;
}

#[derive(new, Error, Debug, Clone, PartialEq)]
pub enum GetOrdersUseCaseError {
    #[error("Limit is exceeded")]
    LimitExceed(i32),
}