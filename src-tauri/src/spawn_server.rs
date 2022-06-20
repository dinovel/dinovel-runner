use crate::app_conf::AppConfigMode;

pub(crate) fn spawn() {
  let config = crate::conf_handler::read_config();
  if AppConfigMode::Url.to_string() == config.mode {
    return;
  }

  spawn_server(config.target);
}

fn spawn_server(relative_path: String) {
  let path = std::path::Path::new(&relative_path);
  if !path.exists() { panic!("Target file does not exist: {}", relative_path); }
  
  let full_path = std::fs::canonicalize(path).unwrap();

  std::process::Command::new(full_path)
    .arg("--port 8000")
    .spawn()
    .expect("Failed to spawn server");
}
