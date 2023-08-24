pub(crate) mod profile;
use profile::Profile;

use std::{
    io::{self, BufRead, BufReader, Read, Write},
    process::{Child, ChildStdin, ChildStdout, Command, Stdio}, collections::VecDeque, time::{Instant, Duration}, thread,
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
            .arg("--simple-io")
            .arg("--threads")
            .arg("4")
            .stderr(Stdio::null())
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
        self.stdin.write(data.as_bytes()).expect("question, imposible");
        self.stdin.flush().unwrap(); //usless ?
    }

    pub fn reponse(&mut self) {
        let reader = &mut self.stdout;

        //because cannot read byte just array
        let mut buffer = [0; 1]; // Buffer to hold a single byte

        let target = "User:"; // <=========================== stop DETECTION
        let window_size: usize = target.len();
        let mut window: VecDeque<u8> = VecDeque::with_capacity(window_size);


        for _ in 0..window_size{
            window.push_back(b' ')
        }

        loop {
            //read byte by byte
            let start_time = Instant::now();
            match reader.read_exact(&mut buffer) {
                Ok(_) => {
                    let character = buffer[0];

                    print!("{}", character as char) ;//<= need to store into somthing to detect line and return just line
                    io::stdout().flush().expect("Failed to flush stdout");

                    //remove last carac
                    window.pop_front();
                    window.push_back(character);

                    // Check if the buffer contains the target string
                    let buffer_str: String = window.iter().map(|&b| b as char).collect();
                    if buffer_str.contains(target) {
                        //detect END TOKEN
                        break;//QUIT when detected
                    }
                    

                }

                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // WouldBlock indicates that the read operation would block.
                    // Check if the timeout has been exceeded.
                    if start_time.elapsed() >= Duration::from_secs(1) {
                        // return Err(io::Error::new(io::ErrorKind::TimedOut, "Read operation timed out"));
                    }
                    // Sleep briefly to avoid busy-waiting
                    thread::sleep(Duration::from_millis(1));   
                }
                Err(e) => {
                    eprintln!("Error reading from child process: {}", e);
                    break;
                }
            }
        }
    }



    pub fn interaction(&mut self,question:&str) {
        let formated = format!("{}\n",question);
        self.question(formated.as_str());
        self.reponse();
    }



    pub fn quit(mut self) {
        self.process.kill().expect("cannot kill");
        self.process
            .wait()
            .expect("je ne peut pas attendre que je meur apres etre mort !");
    }

    fn read_and_purge_stdout(reader: &mut BufReader<ChildStdout>, argv: Vec<String>) {
        eprintln!("Wait Llama to start");
        let prompt_org = argv
            .last()
            .expect("le derner argument est pas accessible ?");
        let last_line = prompt_org
            .lines()
            .rev()
            .nth(1)
            .expect("dernierre line non accessible");

        for line in reader.lines() {
            match line {
                Ok(line_content) => {
                    // print!("{}", line_content);
                    io::stdout().flush().expect("Failed to flush stdout");
                    if line_content.trim() == last_line {
                        break;
                    }
                }
                Err(error) => eprintln!("Error reading line: {}", error),
            }
        }
    }
    //in future start in base with resume of
}
