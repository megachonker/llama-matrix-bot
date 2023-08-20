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
    model: String,
    interactive: bool,
    reverse_prompt: String,
    prompt: String,
}

impl Config {
    pub fn new(path: String) -> Self {
        Config::read_config_from_file(path).expect("FUYOU")
    }

    pub fn build_cmd(&self) -> Vec<String> {
        let cmd_arg = self.command_args.clone();
        vec![
            self.path.clone(),
            "--model".to_string(),
            cmd_arg.model.clone(),
            if cmd_arg.interactive {
                "--interactive".to_string()
            } else {
                "".to_string()
            },
            "--reverse_prompt".to_string(),
            cmd_arg.reverse_prompt.clone(),
            "--prompt".to_string(),
            cmd_arg.prompt.clone(),
        ]
    }

    fn read_config_from_file(path: String) -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let config: Config = serde_yaml::from_str(&contents)?;

        Ok(config)
    }
}

//     // let conf = read_config_from_file().expect("failed to read conf file");

// // fn lunch_LLM(command_args: &CommandArgs, program_executable: &str) -> Child {
// //     let mut cmd = Command::new(program_executable);

// //     cmd.arg("--model").arg(&command_args.model);
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
