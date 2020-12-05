use crate::actor::{room, user, GameId, RoomId, UserId};
use actix::prelude::*;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub struct Server {
    room: HashMap<RoomId, Addr<room::Room>>,
    counter: u64,
    users: HashMap<UserId, Addr<user::User>>,
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Option<Addr<room::Room>>")]
pub struct GetRoom(pub RoomId);

impl Handler<GetRoom> for Server {
    type Result = Option<Addr<room::Room>>;

    fn handle(&mut self, msg: GetRoom, _: &mut Self::Context) -> Self::Result {
        self.room.get(&msg.0).cloned()
    }
}

#[derive(Clone, Message)]
#[rtype(result = "RoomId")]
pub struct MakeRoom;

impl Handler<MakeRoom> for Server {
    type Result = RoomId;

    fn handle(&mut self, _: MakeRoom, ctx: &mut Self::Context) -> Self::Result {
        let room_id = RoomId(self.generate_uuid("room"));
        self.room
            .insert(room_id, room::Room::new(room_id, ctx.address()).start());
        room_id
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct RemoveRoom(pub RoomId);

impl Handler<RemoveRoom> for Server {
    type Result = ();

    fn handle(&mut self, msg: RemoveRoom, _: &mut Self::Context) -> Self::Result {
        self.room.remove(&msg.0);
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Connect(pub UserId, pub Addr<user::User>);

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        self.users.insert(msg.0, msg.1);
    }
}

#[derive(Clone, Message)]
#[rtype(result = "Option<Addr<user::User>>")]
pub struct GetUser(pub UserId);

impl Handler<GetUser> for Server {
    type Result = Option<Addr<user::User>>;

    fn handle(&mut self, msg: GetUser, _: &mut Self::Context) -> Self::Result {
        self.users.get(&msg.0).cloned()
    }
}

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct Disconnect(pub UserId);

impl Handler<Disconnect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.users.remove(&msg.0);
    }
}

#[derive(Clone, Message)]
#[rtype(result = "GameId")]
pub struct MakeGameId;

impl Handler<MakeGameId> for Server {
    type Result = GameId;

    fn handle(&mut self, _: MakeGameId, _: &mut Self::Context) -> Self::Result {
        GameId(self.generate_uuid("game"))
    }
}

impl Server {
    pub fn new() -> Server {
        Server {
            room: HashMap::new(),
            counter: 0,
            users: HashMap::new(),
        }
    }

    pub fn generate_uuid(&mut self, tag: &str) -> Uuid {
        self.counter += 1;
        Uuid::new_v5(
            &Uuid::NAMESPACE_OID,
            format!(
                "{}-{}-{}",
                tag,
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos(),
                self.counter,
            )
            .as_ref(),
        )
    }
}