use sdk::{cosmwasm_ext::Response as CwResponse, cosmwasm_std::to_binary};
use serde::Serialize;

use crate::{error, message::Response as MessageResponse};

#[inline]
pub fn empty_response() -> CwResponse {
    response_only_messages(MessageResponse::default())
}

#[inline]
pub fn response<T, E>(response: &T) -> Result<CwResponse, E>
where
    T: Serialize + ?Sized,
    error::Error: Into<E>,
{
    response_with_messages(response, MessageResponse::default())
}

pub fn response_only_messages<M>(messages: M) -> CwResponse
where
    M: Into<MessageResponse>,
{
    let MessageResponse {
        messages,
        events: may_events,
    } = messages.into();

    let cw_resp: CwResponse = messages
        .into_iter()
        .fold(Default::default(), |res, msg| res.add_submessage(msg));

    if let Some(events) = may_events {
        cw_resp.add_event(events.into())
    } else {
        cw_resp
    }
}

pub fn response_with_messages<T, M, E>(response: &T, messages: M) -> Result<CwResponse, E>
where
    T: Serialize + ?Sized,
    error::Error: Into<E>,
    M: Into<MessageResponse>,
{
    to_binary(response)
        .map_err(error::Error::from)
        .map_err(Into::into)
        .map(|resp_bin| response_only_messages(messages).set_data(resp_bin))
}

#[cfg(test)]
mod test {
    use sdk::{
        cosmwasm_ext::{CosmosMsg, Response},
        cosmwasm_std::{to_binary, Event, WasmMsg},
    };

    use crate::{
        batch::{Batch, Emitter},
        emit::Emit,
        error::Error,
        message::Response as MessageResponse,
    };

    const TY1: &str = "E_TYPE";
    const KEY1: &str = "my_event_key";
    const KEY2: &str = "my_other_event_key";
    const VALUE1: &str = "my_event_value";
    const VALUE2: &str = "my_other_event_value";

    #[test]
    fn no_events() {
        let mut b = Batch::default();
        b.schedule_execute_no_reply(CosmosMsg::Wasm(WasmMsg::ClearAdmin {
            contract_addr: "".to_string(),
        }));
        let resp: Response = super::response_only_messages(b);
        assert_eq!(1, resp.messages.len());
        assert_eq!(0, resp.attributes.len());
        assert_eq!(0, resp.events.len());
        assert_eq!(None, resp.data);
    }

    #[test]
    fn emit() {
        let e = Emitter::of_type(TY1).emit(KEY1, VALUE1);
        let resp: Response = super::response_only_messages(e);
        assert_eq!(1, resp.events.len());
        let exp = Event::new(TY1).add_attribute(KEY1, VALUE1);
        assert_eq!(exp, resp.events[0]);
        assert_eq!(0, resp.messages.len());
        assert_eq!(0, resp.attributes.len());
        assert_eq!(None, resp.data);
    }

    #[test]
    fn emit_two_attrs() {
        let emitter = Emitter::of_type(TY1).emit(KEY1, VALUE1).emit(KEY2, VALUE2);
        let resp: Response = super::response_only_messages(emitter);
        assert_eq!(1, resp.events.len());
        let exp = Event::new(TY1)
            .add_attribute(KEY1, VALUE1)
            .add_attribute(KEY2, VALUE2);
        assert_eq!(exp, resp.events[0]);
        assert_eq!(0, resp.messages.len());
        assert_eq!(0, resp.attributes.len());
        assert_eq!(None, resp.data);
    }

    #[test]
    fn msgs_len() {
        let mut b = Batch::default();
        b.schedule_execute_no_reply(CosmosMsg::Wasm(WasmMsg::ClearAdmin {
            contract_addr: "".into(),
        }));
        b.schedule_execute_no_reply(CosmosMsg::Wasm(WasmMsg::UpdateAdmin {
            contract_addr: "".into(),
            admin: "".into(),
        }));
        assert_eq!(2, b.len());
        assert!(!b.is_empty());

        let resp: Response = super::response_only_messages(b);
        assert_eq!(2, resp.messages.len());
        assert_eq!(None, resp.data);
    }

    #[test]
    fn resp() {
        let ret: u16 = 45;
        let resp: Response = super::response::<_, Error>(&ret).unwrap();
        assert_eq!(0, resp.messages.len());
        assert_eq!(0, resp.attributes.len());
        assert_eq!(0, resp.events.len());
        assert_eq!(Some(to_binary(&ret).unwrap()), resp.data);
    }

    #[test]
    fn resp_with_messages() {
        let ret: u16 = 435;
        let mut b = Batch::default();
        b.schedule_execute_no_reply(CosmosMsg::Wasm(WasmMsg::ClearAdmin {
            contract_addr: "".to_string(),
        }));
        let emitter = Emitter::of_type(TY1).emit(KEY1, VALUE1).emit(KEY2, VALUE2);
        let resp: Response = super::response_with_messages::<_, _, Error>(
            &ret,
            MessageResponse::messages_with_events(b, emitter),
        )
        .unwrap();
        assert_eq!(1, resp.messages.len());
        assert_eq!(0, resp.attributes.len());
        assert_eq!(1, resp.events.len());
        assert_eq!(Some(to_binary(&ret).unwrap()), resp.data);
    }
}
