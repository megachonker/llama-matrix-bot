pub(crate) mod profile;
use profile::Profile;

use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

pub struct Worker {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    process: Child,
}

impl Worker {
    pub fn new(profile: Profile) -> Self {
        let lunch_args = profile.create_lungh_arg();
        //prompt need to be last arg !
        let mut process = Command::new(&lunch_args.first().expect("LOL ces vide"))
            .args(&lunch_args[1..])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to lunch LLaMa instance");

        //crée un buffer de lecture
        let stdout = process.stdout.take().expect("error");
        let mut stdout = BufReader::new(stdout);

        //purge le buffer
        Self::read_and_purge_stdout(&mut stdout, lunch_args);

        Self {
            stdin: process.stdin.take().expect("cursed"),
            stdout: stdout,
            process,
        }
    }

    pub fn question(&mut self, data: &str) {
        //faire des check ici
        self.stdin.write(data.as_bytes());
    }

    pub fn reponse(&mut self) {
        //a check
        let reader = &mut self.stdout;

        for line in reader.lines() {
            match line {
                Ok(line_content) => println!("{}", line_content),
                Err(error) => eprintln!("Error reading line: {}", error),
            }
        }
    }

    pub fn quit(mut self) {
        self.process.kill().expect("cannot kill");
        self.process
            .wait()
            .expect("je ne peut pas attendre que je meur apres etre mort !");
    }

    //GPTed func
    fn read_and_purge_stdout(reader: &mut BufReader<ChildStdout>, target_line: Vec<String>) {
        let target_line = target_line.last().expect("le derner est pas accessible ?");
        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    if line_content.trim() == target_line.lines().next().unwrap() {
                        break;
                    }
                }
                Err(error) => eprintln!("Error reading line: {}", error),
            }
        }
    }
    //in future start in base with resume of
}
