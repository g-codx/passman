use secrecy::zeroize::Zeroize;

#[derive(Default)]
pub struct Security {
    pub change_password_open: bool,
    pub settings_open: bool,
    pub current: String,
    pub new: String,
    pub confirm: String,
    pub auto_lock_secs: u64,
}

impl Security {
    pub fn clear_password_fields(&mut self) {
        self.current.zeroize();
        self.new.zeroize();
        self.confirm.zeroize();
        self.current.shrink_to_fit();
        self.new.shrink_to_fit();
        self.confirm.shrink_to_fit();
    }
}
