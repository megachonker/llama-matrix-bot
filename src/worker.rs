pub(crate) mod profile;
use profile::Profile;

pub struct Worker {}

impl Worker {
    pub fn new(profile: Profile) -> Self {
        let lunch_arg = profile.build_cmd();
        Self {}
    }
}
