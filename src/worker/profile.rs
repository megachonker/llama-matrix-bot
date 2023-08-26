use crate::config::Config;

#[derive(Default)]
pub enum Profile {
    #[default]
    base,
    raw(Vec<String>),
    from_config(Config),
}

impl Profile {
    pub fn create_lungh_arg(self) -> Vec<String> {
        //maybe build a trait
        return match self {
            Profile::raw(val) => val,
            Profile::from_config(conf) => conf.build_cmd(),
            Profile::base => Config::new("config_test.yaml".to_string()).build_cmd(), //file are dependent of directory
            //fast
            //short answer
            //long
        };
    }

    //get other variable, like timeout
    //stop tocken ect
}

#[cfg(test)]
mod tests {

    use crate::{config::Config, worker::profile::Profile};

    #[test]
    fn raw() {
        let args = vec!["OwO".to_string()];
        assert_eq!(args, Profile::raw(args.clone()).create_lungh_arg())//SHITI
    }
    #[test]
    fn from_config() {
        let args = vec!["--OwO".to_string()];
        let azer: Config = Default::default();
        assert_eq!(args.clone(), Profile::from_config(azer).create_lungh_arg())
    }
    #[test]
    fn base() {
        assert_eq!(true, true)
    }
}
