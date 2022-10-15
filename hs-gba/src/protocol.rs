use crossbeam_channel::{
    self,
    Sender,
    Receiver
};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Message {
    Reset,
    Run,
    Pause,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Notification {
}

#[derive(Debug, Clone)]
pub struct EmulatorChannels {
    messages: Receiver<Message>,
    notifications: Sender<Notification>,
}

impl EmulatorChannels {
    pub fn send(&mut self, notification: Notification) {
        let _ = self.notifications.send(notification);
    }

    pub fn try_receive(&mut self) -> Option<Message> {
        self.messages.try_recv().ok()
    }

    pub fn wait(&mut self) -> Option<Message> {
        self.messages.recv().ok()
    }
}

#[derive(Debug, Clone)]
pub struct FrontendChannels {
    messages: Sender<Message>,
    notifications: Receiver<Notification>,
}

impl FrontendChannels {
    pub fn send(&mut self, message: Message) {
        let _ = self.messages.send(message);
    }

    pub fn receive(&mut self) -> Option<Notification> {
        self.notifications.try_recv().ok()
    }

    pub fn wait(&mut self) -> Option<Notification> {
        self.notifications.recv().ok()
    }
}

pub fn new_channels() -> (EmulatorChannels, FrontendChannels) {
    let (msg_sender, msg_receiver) = crossbeam_channel::unbounded();
    let (notif_sender, notif_receiver) = crossbeam_channel::unbounded();

    let emulator_channels = EmulatorChannels {
        messages: msg_receiver,
        notifications: notif_sender,
    };

    let frontend_channels = FrontendChannels {
        messages: msg_sender,
        notifications: notif_receiver,
    };

    (emulator_channels, frontend_channels)
}