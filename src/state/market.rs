use std::collections::HashMap;

use screeps::{
    game::market::{MyOrder, Order},
    MarketResourceType, OrderType, RoomName,
};

#[derive(Debug, Default)]
pub struct MarketState {
    /// Resource types with values and their average price over the last 14 days
    pub resource_history: Option<ResourceHistory>,
    /// Intra-tick cached orders owned by the bot
    pub my_orders: Option<MyOrders>,
    /// Intra tick cached orders owned by anyone
    pub all_orders: Option<AllOrders>,
}

impl MarketState {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn tick_update(&mut self) {
        self.my_orders = None;
        self.all_orders = None;
    }
}

pub type ResourceHistory = HashMap<MarketResourceType, HashMap<u32, u32>>;
pub type MyOrders = HashMap<RoomName, HashMap<MarketResourceType, Vec<MyOrder>>>;
pub type AllOrders = HashMap<MarketResourceType, HashMap<OrderType, Vec<Order>>>;
