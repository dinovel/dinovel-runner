use std::process::Child;

use crate::app_conf::AppConfigMode;

pub(crate) enum SpawnResult {
  Server {
    child: Child,
    port: u16,
  },
  Url(String),
}

impl SpawnResult {
  pub fn to_string(&self) -> String {
    match self {
      SpawnResult::Server { port, .. } => {
        format!("http://localhost:{}", port)
      }
      SpawnResult::Url(url) => url.to_string(),
    }
  }

  pub fn kill(&mut self) -> () {
    match self {
      SpawnResult::Server { child, .. } => {
        child.kill().unwrap();
      }
      _ => {}
    }
  }
}

pub(crate) fn spawn(env: &str) -> SpawnResult {
  let config = crate::conf_handler::read_config(env);
  if AppConfigMode::Url.to_string() == config.mode {
    return SpawnResult::Url(config.target);
  }

  spawn_server(config.target)
}

fn spawn_server(path: String) -> SpawnResult {
  let port = crate::port_scanner::get_free_port().unwrap();
  let child = run_process(path, vec!["--port".to_string(), port.to_string()]);

  SpawnResult::Server {
    child,
    port,
  }
}

fn run_process(relative_path: String, args: Vec<String>) -> Child {
  let path = std::path::Path::new(&relative_path);
  if !path.exists() { panic!("Target file does not exist: {}", relative_path); }
  
  let full_path = std::fs::canonicalize(path).unwrap();

  let mut prg = std::process::Command::new(full_path);

  for arg in args {
    prg.arg(arg);
  }

  prg.spawn().expect("Error while spawning process")
}
