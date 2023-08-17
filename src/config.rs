use serde::Deserialize;
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Config {
    matrix: MatrixConfig,
    path: String,
    pub command_args: CommandArgs,
}

#[derive(Debug, Deserialize, Clone, Default)]
struct MatrixConfig {
    username: String,
    password: String,
    homeserver_url: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct CommandArgs {
    ctx_size: u32,
    temp: f64,
    top_k: u32,
    top_p: f64,
    repeat_last_n: u32,
    batch_size: u32,
    repeat_penalty: f64,
    model: String,
    threads: u32,
    n_predict: u32,
    interactive: bool,
    reverse_prompt: String,
    prompt: String,
}

impl Config {
    pub fn new(path: String) -> Self {
        Config::read_config_from_file(path).expect("FUYOU")
    }

    pub fn build_cmd(&self) -> String {
        self.command_args.to_string()
    }

    fn read_config_from_file(path: String) -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_yaml::from_str(&contents)?;

        Ok(config)
    }
}

// trai to_string for CommandArgs{

// }

impl ToString for CommandArgs {
    fn to_string(&self) -> String {
        format!("--OwO")
    }
}

//     // let conf = read_config_from_file().expect("failed to read conf file");

// // fn lunch_LLM(command_args: &CommandArgs, program_executable: &str) -> Child {
// //     let mut cmd = Command::new(program_executable);

// //     cmd.arg("--ctx_size").arg(command_args.ctx_size.to_string());
// //     cmd.arg("--temp").arg(command_args.temp.to_string());
// //     cmd.arg("--top_k").arg(command_args.top_k.to_string());
// //     cmd.arg("--top_p").arg(command_args.top_p.to_string());
// //     cmd.arg("--repeat_last_n")
// //         .arg(command_args.repeat_last_n.to_string());
// //     cmd.arg("--batch_size")
// //         .arg(command_args.batch_size.to_string());
// //     cmd.arg("--repeat_penalty")
// //         .arg(command_args.repeat_penalty.to_string());
// //     cmd.arg("--model").arg(&command_args.model);
// //     cmd.arg("--threads").arg(command_args.threads.to_string());
// //     cmd.arg("--n_predict")
// //         .arg(command_args.n_predict.to_string());
// //     if command_args.interactive {
// //         cmd.arg("--interactive");
// //     }
// //     cmd.arg("--reverse-prompt")
// //         .arg(&command_args.reverse_prompt);
// //     cmd.arg("--prompt").arg(&command_args.prompt);
// //     cmd.stdin(Stdio::piped())
// //         .stdout(Stdio::piped())
// //         .spawn()
// //         .expect("failed to lunch LLaMa")
// // }
