pub(crate) mod profile;

use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

use profile::Profile;

pub struct Worker {
    stdin: ChildStdin,
    stdout: ChildStdout,
    process: Child,
}

impl Worker {
    pub fn new(profile: Profile) -> Self {
        let lunch_args = profile.create_lungh_arg();

        let mut process = Command::new(&lunch_args[0])
            .args(&lunch_args[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to lunch LLaMa instance");
        Self {
            stdin: process.stdin.take().expect("cursed"),
            stdout: process.stdout.take().expect("error"),
            process,
        }
    }

    pub fn question(&mut self, data: &str) {
        //faire des check ici
        self.stdin.write(data.as_bytes());
    }

    pub fn reponse(&mut self) -> String {
        let reader = BufReader::new(&mut self.stdout);
        reader
            .lines()
            .next()
            .expect("cannot get line")
            .expect("cannot read str")
    }

    pub fn quit(mut self) {
        self.process.kill().expect("cannot kill");
        self.process
            .wait()
            .expect("je ne peut pas attendre que je meur apres etre mort !");
    }

    //in future start in base with resume of
}
