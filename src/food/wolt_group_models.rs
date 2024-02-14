use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub drivers: Vec<Value>,
    #[serde(rename = "expires_in_seconds")]
    pub expires_in_seconds: i64,
    #[serde(rename = "order_details")]
    pub order_details: OrderDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderDetails {
    #[serde(rename = "automatic_rejection_time")]
    pub automatic_rejection_time: AutomaticRejectionTime,
    #[serde(rename = "client_pre_estimate")]
    pub client_pre_estimate: String,
    pub credits: i64,
    pub currency: String,
    #[serde(rename = "delivery_base_price")]
    pub delivery_base_price: i64,
    #[serde(rename = "delivery_comment")]
    pub delivery_comment: String,
    #[serde(rename = "delivery_distance")]
    pub delivery_distance: i64,
    #[serde(rename = "delivery_eta")]
    pub delivery_eta: DeliveryEta,
    #[serde(rename = "delivery_location")]
    pub delivery_location: DeliveryLocation,
    #[serde(rename = "delivery_method")]
    pub delivery_method: String,
    #[serde(rename = "delivery_price")]
    pub delivery_price: i64,
    #[serde(rename = "delivery_price_share")]
    pub delivery_price_share: i64,
    #[serde(rename = "delivery_size_surcharge")]
    pub delivery_size_surcharge: i64,
    #[serde(rename = "delivery_time")]
    pub delivery_time: DeliveryTime,
    pub discounts: Vec<Value>,
    #[serde(rename = "driver_type")]
    pub driver_type: String,
    pub group: Group,
    #[serde(rename = "is_host_paying")]
    pub is_host_paying: bool,
    #[serde(rename = "is_marketplace_v2")]
    pub is_marketplace_v2: bool,
    #[serde(rename = "items_price")]
    pub items_price: i64,
    #[serde(rename = "list_image")]
    pub list_image: String,
    #[serde(rename = "list_image_blurhash")]
    pub list_image_blurhash: String,
    #[serde(rename = "main_image")]
    pub main_image: String,
    #[serde(rename = "main_image_blurhash")]
    pub main_image_blurhash: String,
    #[serde(rename = "order_adjustment_rows")]
    pub order_adjustment_rows: Vec<Value>,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "order_number")]
    pub order_number: String,
    #[serde(rename = "payment_amount")]
    pub payment_amount: i64,
    #[serde(rename = "payment_method")]
    pub payment_method: PaymentMethod,
    #[serde(rename = "payment_name")]
    pub payment_name: String,
    #[serde(rename = "payment_time")]
    pub payment_time: PaymentTime,
    pub payments: Vec<Payment>,
    #[serde(rename = "preorder_status")]
    pub preorder_status: String,
    #[serde(rename = "preorder_time")]
    pub preorder_time: PreorderTime,
    #[serde(rename = "service_fee")]
    pub service_fee: i64,
    #[serde(rename = "show_delivery_preestimate_by_time")]
    pub show_delivery_preestimate_by_time: bool,
    pub status: String,
    pub subscribed: bool,
    pub subtotal: i64,
    pub surcharges: Vec<Value>,
    pub tip: i64,
    #[serde(rename = "tip_config")]
    pub tip_config: TipConfig,
    #[serde(rename = "tip_share")]
    pub tip_share: i64,
    pub tokens: i64,
    #[serde(rename = "total_price")]
    pub total_price: i64,
    #[serde(rename = "total_price_share")]
    pub total_price_share: i64,
    #[serde(rename = "venue_address")]
    pub venue_address: String,
    #[serde(rename = "venue_coordinates")]
    pub venue_coordinates: Vec<f64>,
    #[serde(rename = "venue_country")]
    pub venue_country: String,
    #[serde(rename = "venue_full_address")]
    pub venue_full_address: String,
    #[serde(rename = "venue_id")]
    pub venue_id: String,
    #[serde(rename = "venue_name")]
    pub venue_name: String,
    #[serde(rename = "venue_open")]
    pub venue_open: bool,
    #[serde(rename = "venue_open_on_purchase")]
    pub venue_open_on_purchase: bool,
    #[serde(rename = "venue_phone")]
    pub venue_phone: String,
    #[serde(rename = "venue_product_line")]
    pub venue_product_line: String,
    #[serde(rename = "venue_timezone")]
    pub venue_timezone: String,
    #[serde(rename = "venue_url")]
    pub venue_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomaticRejectionTime {
    #[serde(rename = "$date")]
    pub date: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryEta {
    #[serde(rename = "$date")]
    pub date: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryLocation {
    pub address: String,
    pub city: String,
    pub coordinates: Coordinates,
    pub street: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Coordinates {
    pub coordinates: Vec<f64>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryTime {
    #[serde(rename = "$date")]
    pub date: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub checksum: String,
    pub icon: String,
    pub id: String,
    pub locked: bool,
    #[serde(rename = "modified_at")]
    pub modified_at: ModifiedAt,
    #[serde(rename = "my_member")]
    pub my_member: MyMember,
    pub name: String,
    #[serde(rename = "other_members")]
    pub other_members: Vec<OtherMember>,
    #[serde(rename = "split_payment")]
    pub split_payment: bool,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifiedAt {
    #[serde(rename = "$date")]
    pub date: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MyMember {
    pub comment: String,
    #[serde(rename = "first_name")]
    pub first_name: String,
    #[serde(rename = "guest_id")]
    pub guest_id: String,
    pub image: String,
    #[serde(rename = "is_host")]
    pub is_host: bool,
    #[serde(rename = "item_change_log")]
    pub item_change_log: Vec<Value>,
    pub items: Vec<Item>,
    #[serde(rename = "items_price")]
    pub items_price: i64,
    #[serde(rename = "total_share")]
    pub total_share: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub count: i64,
    #[serde(rename = "end_amount")]
    pub end_amount: i64,
    pub id: String,
    pub name: String,
    pub options: Vec<Option>,
    pub price: i64,
    #[serde(rename = "row_number")]
    pub row_number: i64,
    #[serde(rename = "skip_on_refill")]
    pub skip_on_refill: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Option {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub values: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value {
    pub count: i64,
    pub id: String,
    pub name: String,
    pub price: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherMember {
    pub comment: String,
    #[serde(rename = "first_name")]
    pub first_name: String,
    pub image: String,
    #[serde(rename = "is_host")]
    pub is_host: bool,
    #[serde(rename = "item_change_log")]
    pub item_change_log: Vec<Value>,
    pub items: Vec<Item2>,
    #[serde(rename = "items_price")]
    pub items_price: i64,
    #[serde(rename = "last_name")]
    pub last_name: String,
    #[serde(rename = "total_share")]
    pub total_share: i64,
    #[serde(rename = "user_id")]
    pub user_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item2 {
    pub count: i64,
    #[serde(rename = "end_amount")]
    pub end_amount: i64,
    pub id: String,
    pub name: String,
    pub options: Vec<Option2>,
    pub price: i64,
    #[serde(rename = "row_number")]
    pub row_number: i64,
    #[serde(rename = "skip_on_refill")]
    pub skip_on_refill: bool,
    #[serde(rename = "substitution_settings")]
    pub substitution_settings: Option<SubstitutionSettings>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Option2 {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub values: Vec<Value2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Value2 {
    pub count: i64,
    pub id: String,
    pub name: String,
    pub price: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubstitutionSettings {
    #[serde(rename = "allowed_items")]
    pub allowed_items: Vec<Value>,
    #[serde(rename = "is_allowed")]
    pub is_allowed: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethod {
    pub id: String,
    pub provider: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTime {
    #[serde(rename = "$date")]
    pub date: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub amount: i64,
    pub method: Method,
    pub name: String,
    #[serde(rename = "payment_time")]
    pub payment_time: PaymentTime2,
    #[serde(rename = "refunded_amount")]
    pub refunded_amount: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    pub id: String,
    pub provider: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentTime2 {
    #[serde(rename = "$date")]
    pub date: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreorderTime {
    #[serde(rename = "$date")]
    pub date: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TipConfig {
    #[serde(rename = "adjustments_enabled")]
    pub adjustments_enabled: bool,
    #[serde(rename = "allow_custom_tip")]
    pub allow_custom_tip: bool,
    #[serde(rename = "allowed_payment_methods")]
    pub allowed_payment_methods: Vec<String>,
    #[serde(rename = "max_amount")]
    pub max_amount: i64,
    #[serde(rename = "min_amount")]
    pub min_amount: i64,
    #[serde(rename = "order_review_tipping_enabled")]
    pub order_review_tipping_enabled: bool,
    #[serde(rename = "tip_amounts")]
    pub tip_amounts: Vec<i64>,
}
