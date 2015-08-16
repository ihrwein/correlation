use std::sync::mpsc;
use std::thread;
use std::result::Result;

use action::ActionHandlers;
use {config, context, Message, MiliSec, Response, Timer};
use condition::Condition;
use dispatcher::request::{InternalRequest, Request, RequestHandler};
use dispatcher::reactor::RequestReactor;
use dispatcher::demux::Demultiplexer;
use dispatcher::handlers;
use reactor::Reactor;

const TIMER_STEP: MiliSec = 100;

pub struct Correlator {
    action_handlers: ActionHandlers,
    dispatcher_input_channel: mpsc::Sender<Request<Message>>,
    dispatcher_output_channel: mpsc::Receiver<Response>,
    dispatcher_thread_handle: thread::JoinHandle<()>,
    exits_received: u32
}

impl Correlator {
    pub fn new(contexts: Vec<config::Context>, action_handlers: ActionHandlers) -> Correlator {
        let (dispatcher_input_channel, rx) = mpsc::channel();
        let (dispatcher_output_channel_tx, dispatcher_output_channel_rx) = mpsc::channel();
        let _ = Timer::from_chan(TIMER_STEP, dispatcher_input_channel.clone());

        let handle = thread::spawn(move || {
            let dmux = Demultiplexer::new(rx);
            let exit_condition = Condition::new(false);
            let mut reactor = RequestReactor::new(dmux, exit_condition.clone());
            let exit_handler = Box::new(handlers::exit::ExitEventHandler::new(exit_condition));
            reactor.register_handler(exit_handler);

            let timer_event_handler = Box::new(handlers::timer::TimerEventHandler::new());
            //event_handler.register_handler(timer_event_handler);

            for i in contexts.into_iter() {
                let context: context::Context = i.into();
                let event_handler: Box<context::event::EventHandler<InternalRequest>> = context.into();
            }

            //let mut dispatcher = Dispatcher::new(contexts, dispatcher_output_channel_tx);
            //dispatcher.start_loop(rx);
        });

        Correlator {
            action_handlers: action_handlers,
            dispatcher_input_channel: dispatcher_input_channel,
            dispatcher_output_channel: dispatcher_output_channel_rx,
            dispatcher_thread_handle: handle,
            exits_received: 0
        }
    }

    pub fn push_message(&mut self, message: Message) -> Result<(), mpsc::SendError<Request<Message>>> {
        self.consume_results();
        self.dispatcher_input_channel.send(Request::Message(message))
    }

    fn consume_results(&mut self) {
        for i in self.dispatcher_output_channel.try_recv() {
            if let Response::Event(result) = i {
                self.action_handlers.handle(result);
            }
        }
    }

    pub fn stop(mut self) -> thread::Result<()> {
        self.consume_results();
        self.stop_dispatcher();
        self.dispatcher_thread_handle.join()
    }

    fn stop_dispatcher(&mut self) {
        let _ = self.dispatcher_input_channel.send(Request::Exit);
        let _ = self.wait_for_dispatcher_to_exit();
    }

    fn wait_for_dispatcher_to_exit(&mut self) -> Result<(), ()> {
        loop {
            let value = self.dispatcher_output_channel.recv();
            match value {
                Ok(value) => {
                    try!(self.handle_command(value))
                },
                _ => {}
            }
        }
    }

    fn handle_command(&mut self, command: Response) -> Result<(), ()> {
        match command {
            Response::Event(result) => self.action_handlers.handle(result),
            Response::Exit => {
                if self.handle_exit_command() {
                    return Err(());
                }
            }
        }
        Ok(())
    }

    fn handle_exit_command(&mut self) -> bool {
        let _ = self.dispatcher_input_channel.send(Request::Exit);
        self.exits_received += 1;
        self.exits_received >= 1
    }
}
