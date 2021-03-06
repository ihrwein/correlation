// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::sync::mpsc::Sender;

use action::Alert;
use reactor::Event;
use self::response::ResponseSender;

pub mod demux;
pub mod handlers;
pub mod response;
pub mod request;
pub mod reactor;

#[derive(Debug, Clone)]
pub enum Response {
    Exit,
    Alert(Alert),
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub enum ResponseHandle {
    Exit,
    Alert,
}

impl Event for Response {
    type Handle = ResponseHandle;
    fn handle(&self) -> Self::Handle {
        match *self {
            Response::Exit => ResponseHandle::Exit,
            Response::Alert(_) => ResponseHandle::Alert,
        }
    }
}

impl ResponseSender for Sender<Response> {
    fn send_response(&mut self, response: Response) {
        let _ = self.send(response);
    }
}
