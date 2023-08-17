use crate::config::Config;

#[derive(Default)]
pub enum Profile {
    #[default]
    base,
    from_config(Config),
    raw(String)
}

impl Profile {
    pub fn build_cmd(self) -> String { //maybe build a trait
        return match self {
            Profile::raw(val) => val,  
            Profile::from_config(conf) => conf.build_cmd(),
            Profile::base => Config::new("config.yaml".to_string()).build_cmd(),
        };
    }
}


#[cfg(test)]
mod tests{

    #[test]
    fn raw(){

    }
    #[test]
    fn from_config(){

    }
    #[test]
    fn base(){

    }
}