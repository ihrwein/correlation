use dispatcher::request::{Request, RequestHandler};
use condition::Condition;
use reactor::EventHandler;

pub struct ExitHandler{
    condition: Condition,
    stops: u32
}

impl ExitHandler {
    pub fn new(condition: Condition) -> ExitHandler {
        ExitHandler {
            condition: condition,
            stops: 0
        }
    }
}

impl EventHandler<Request> for ExitHandler {
    type Handler = RequestHandler;
    fn handle_event(&mut self, event: Request) {
        if let Request::Exit = event {
            self.stops += 1;

            if self.stops >= 2 {
                self.condition.activate();
            }
        } else {
            unreachable!("An ExitHandler should only receive Exit events");
        }
    }
    fn handler(&self) -> Self::Handler {
        RequestHandler::Exit
    }
}
