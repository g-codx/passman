#[derive(Default)]
pub struct Command(Option<Cmd>);

pub enum Cmd {
    NewEntity,
    UpdateEntity,
    RemoveEntry(usize),
    EditorNew,
    EditorUpdate(usize),
    Backup,
    Export,
    Lock,
    OpenChangePassword,
    OpenSettings,
}

impl Command {
    pub fn set(&mut self, cmd: Cmd) {
        if self.0.is_none() {
            self.0 = Some(cmd);
        }
    }

    pub fn take(&mut self) -> Option<Cmd> {
        self.0.take()
    }
}
