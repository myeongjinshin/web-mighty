use crate::prelude::*;
use crate::ws::session::{Context, SessionTrait};
use types::{ListToClient, ListToServer};

pub struct List;

impl SessionTrait for List {
    type Sender = ListToServer;

    fn tag() -> &'static str {
        "list"
    }

    fn receive(&mut self, msg: String, _: &Context<Self>) -> (&str, JsValue) {
        let msg: ListToClient = serde_json::from_str(&*msg).unwrap();
        ("list", JsValue::from_serde(&msg).unwrap())
    }
}
