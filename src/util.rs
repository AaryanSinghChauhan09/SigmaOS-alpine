//! alpine model — text-based email client.

#[derive(Debug, Clone)]
pub struct Mailbox {
    pub path: String,
    pub messages: Vec<Message>,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub from: String,
    pub subject: String,
    pub flags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub mailboxes: Vec<Mailbox>,
}

impl Default for Model {
    fn default() -> Self { Self { mailboxes: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_mailbox(&mut self, path: &str) -> bool {
        if self.mailboxes.iter().any(|m| m.path == path) { return false; }
        self.mailboxes.push(Mailbox { path: path.into(), messages: vec![] });
        true
    }

    pub fn append(&mut self, mailbox: &str, msg: Message) -> bool {
        if let Some(m) = self.mailboxes.iter_mut().find(|m| m.path == mailbox) {
            m.messages.push(msg); true
        } else { false }
    }

    pub fn list(&self) -> &[Mailbox] { &self.mailboxes }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mailbox_append() {
        let mut a = Model::new();
        assert!(a.add_mailbox("/var/mail/user"));
        assert!(a.append("/var/mail/user", Message { id: "1".into(), from: "a@b.com".into(), subject: "Hi".into(), flags: vec![] }));
        assert_eq!(a.list()[0].messages.len(), 1);
    }
}
