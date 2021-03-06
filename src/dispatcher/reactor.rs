// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::collections::BTreeMap;

use context::ContextMap;
use dispatcher::demux::Demultiplexer;
use dispatcher::request::{RequestHandle, Request};
use reactor::{Event, EventDemultiplexer, EventHandler, Reactor, SharedData};
use dispatcher::response::ResponseSender;

#[allow(type_complexity)]
pub struct RequestReactor {
    handlers: BTreeMap<RequestHandle, Box<for<'a> EventHandler<Request, SharedData<'a>>>>,
    demultiplexer: Demultiplexer<Request>,
    pub context_map: ContextMap,
    responder: Box<ResponseSender>,
}

impl RequestReactor {
    pub fn new(demultiplexer: Demultiplexer<Request>,
               context_map: ContextMap,
               responder: Box<ResponseSender>)
               -> RequestReactor {
        RequestReactor {
            demultiplexer: demultiplexer,
            context_map: context_map,
            handlers: BTreeMap::new(),
            responder: responder,
        }
    }
}

impl Reactor for RequestReactor {
    type Event = Request;
    fn handle_events(&mut self) {
        let mut shared_data = SharedData::new(&mut self.context_map, &mut *self.responder);
        while let Some(request) = self.demultiplexer.select() {
            trace!("RequestReactor: got event");
            if let Some(handler) = self.handlers.get_mut(&request.handle()) {
                handler.handle_event(request, &mut shared_data);
            } else {
                trace!("RequestReactor: no handler found for event");
            }
        }
    }
    fn register_handler(&mut self,
                        handler: Box<for<'a> EventHandler<Self::Event, SharedData<'a>>>) {
        self.handlers.insert(handler.handle(), handler);
    }
    fn remove_handler_by_handle(&mut self, handler: &RequestHandle) {
        self.handlers.remove(handler);
    }
}
