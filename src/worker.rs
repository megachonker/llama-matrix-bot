#[derive(Default)]
pub enum Profile {
    #[default]
    base,
}

pub struct Worker {}

impl Worker {
    pub fn new(profile: Profile) -> Self {
        Worker::build_cmd(profile);
        Self {}
    }

    fn build_cmd(profile: Profile) -> String {
        return match profile {
            Profile::base => String::from("azer"),
        };
    }
}
