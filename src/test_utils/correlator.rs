// Copyright (c) 2016 Tibor Benke <ihrwein@gmail.com>
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use std::sync::mpsc::Sender;

use dispatcher::request::Request;
use action::Alert;
use correlator::AlertHandler;

pub struct MockAlertHandler;

impl AlertHandler<Vec<Alert>> for MockAlertHandler {
    fn on_alert(&mut self, alert: Alert, _: &mut Sender<Request>, extra_data: &mut Vec<Alert>) {
        extra_data.push(alert);
    }
}
