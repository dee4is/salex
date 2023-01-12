pub mod order {
    include!(concat!(env!("OUT_DIR"), "/proto.order.rs"));
}
pub mod manager {
    include!(concat!(env!("OUT_DIR"), "/proto.manager.rs"));
}
pub mod customer {
    include!(concat!(env!("OUT_DIR"), "/proto.customer.rs"));
}
pub mod product {
    include!(concat!(env!("OUT_DIR"), "/proto.product.rs"));
}
pub mod organization {
    include!(concat!(env!("OUT_DIR"), "/proto.organization.rs"));
}
pub mod warehouse {
    include!(concat!(env!("OUT_DIR"), "/proto.warehouse.rs"));
}
