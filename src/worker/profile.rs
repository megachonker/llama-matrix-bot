#[derive(Default)]
pub enum Profile {
    #[default]
    base,
    // CmdArg(CommandArgs), <== need implement trai    
    raw(String)
}

impl Profile {
    pub fn build_cmd(self) -> String {
        return match self {
            Profile::raw(val) => val,  
            Profile::base => String::from("azer"),
        };
    }
}