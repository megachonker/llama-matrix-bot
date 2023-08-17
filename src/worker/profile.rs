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
            Profile::base => Config::new("config.yaml".to_string()).build_cmd(),//file are dependent of directory
        };
    }
}


#[cfg(test)]
mod tests{

    use crate::{worker::profile::Profile, config::Config};


    #[test]
    fn raw(){
        assert_eq!("OwO".to_string(),Profile::raw("OwO".to_string()).build_cmd())
    }
    #[test]
    fn from_config(){
        let azer:Config=Default::default();
        assert_eq!("--OwO".to_string(),Profile::from_config(azer).build_cmd())
    }
    #[test]
    fn base(){
        assert_eq!(true,true)
    }
}