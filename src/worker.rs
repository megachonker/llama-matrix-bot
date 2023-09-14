pub(crate) mod profile;
use profile::Profile;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    process::{Child, ChildStdin, ChildStdout},
};

use std::{
    collections::VecDeque,
    io::{self, Write},
    process::Stdio,
    time::Duration,
};

pub struct Worker {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    process: Child,
}

impl Worker {
    pub async fn new(profile: Profile) -> Self {
        let lunch_args = profile.create_lungh_arg();
        //need to retrive other variable from profile 

        //prompt need to be last arg !
        let mut process = tokio::process::Command::new(&lunch_args.first().expect("LOL ces vide"))
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
        let mut buffreader = BufReader::new(stdout);

        //purge le buffer
        Self::read_and_purge_stdout(&mut buffreader, lunch_args).await;

        Self {
            stdin: process.stdin.take().expect("cursed"),
            stdout: buffreader,
            process,
        }
    }

    async fn question(&mut self, data: &str) {
        //faire des check ici
        print!("{}", data);
        self.stdin
            .write(data.as_bytes())
            .await
            .expect("question, imposible");
    }

    async fn reponse(&mut self) -> Vec::<char>{

        let mut answer = Vec::<char>::new();

        let reader = &mut self.stdout;

        //because cannot read byte just array
        let mut buffer = [0; 1]; // Buffer to hold a single byte

        let target = "User:"; // <=========================== stop DETECTION
        let window_size: usize = target.len();
        let mut window: VecDeque<u8> = VecDeque::with_capacity(window_size);

        //igniore the name and fill bufer
        for _ in 0..window_size {
            reader
                .read_exact(&mut buffer)
                .await
                .expect("first caract err");
            print!("{}", buffer[0] as char);
            window.push_back(b' ')
        }

        //whait first prediction of bot befort starting
        reader
            .read_exact(&mut buffer)
            .await
            .expect("first caract err");
        print!("{}", buffer[0] as char); //<= need to store into somthing to detect line and return just line
        answer.push(buffer[0] as char);

        loop {
            //Get imput
            let ret = tokio::select! {
                opt = reader.read_exact(&mut buffer) => Some(opt),
                _ = tokio::time::sleep(Duration::from_secs(20)) =>None, //<============== Wait NEed to cutsome value
            };

            //Process input
            match ret {
                Some(Ok(_)) => {
                    let character = buffer[0];
                    answer.push(character as char);
                    print!("{}", character as char); //<= need to store into somthing to detect line and return just line
                    io::stdout().flush().expect("Failed to flush stdout");

                    //remove last carac
                    window.pop_front();
                    window.push_back(character);

                    // Check if the buffer contains the target string
                    let buffer_str: String = window.iter().map(|&b| b as char).collect();
                    if buffer_str.contains(target) {
                        //detect END TOKEN
                        break; //QUIT when detected
                    }
                }
                Some(Err(e)) => {
                    eprintln!("error IO {:?}",e);
                    break;
                }
                None => {
                    eprintln!("!!bot stuck Abord!!");
                    break;
                }
            }

            //read byte by byte

            //
        }
        answer
    }
    // <S: AsRef<str>>(input: S)
    pub async fn interaction(&mut self, question: &str) -> String{
        let formated = format!("{}\n", question);
        self.question(formated.as_str()).await;
        self.reponse().await.iter().collect()
    }

    pub async fn quit(mut self) {
        self.process.kill().await.expect("cannot kill");
        // self.process.wait().await.expect("je ne peut pas attendre que je meur apres etre mort !");
    }

    async fn read_and_purge_stdout(reader: &mut BufReader<ChildStdout>, argv: Vec<String>) {
        eprintln!("Wait Llama to start");
        let prompt_org = argv
            .last()
            .expect("le derner argument est pas accessible ?");
        let last_line = prompt_org
            .lines()
            .rev()
            .nth(1)
            .expect("dernierre line non accessible");
        let mut line = reader.lines();

        loop {
            match line.next_line().await {
                Ok(line_content) => {
                    if line_content.expect("imposible de readline").trim() == last_line {
                        break;
                    }
                }
                Err(error) => eprintln!("Error reading line: {}", error),
            }
        }
    }
    //in future start in base with resume of
}
