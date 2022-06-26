use serde_json::Value;

use crate::app_conf::AppConfig;

const CONFIG_FILE: &str = ".dnr";

pub(crate) fn read_config(env: &str) -> AppConfig {
  ensure_updated(env);

  return read_safe_config(env);
}

/* Ensure .conf.json is updated to latest structure */
fn ensure_updated(env: &str) -> () {
  let json = read_dyn_config(&env);
  let mut config = AppConfig::default();

  match json.get("title") {
    Some(v) => config.title = v.as_str().unwrap().to_string(),
    None => {}
  };

  match json.get("width") {
    Some(v) => config.width = v.as_i64().unwrap(),
    None => {}
  };

  match json.get("height") {
    Some(v) => config.height = v.as_i64().unwrap(),
    None => {}
  };

  match json.get("fullscreen") {
    Some(v) => config.fullscreen = v.as_bool().unwrap(),
    None => {}
  };

  match json.get("mode") {
    Some(v) => config.mode = v.as_str().unwrap().to_string(),
    None => {}
  };

  match json.get("target") {
    Some(v) => config.target = v.as_str().unwrap().to_string(),
    None => {}
  };

  write_config(config, env);
}

fn write_config(config: AppConfig, env: &str) -> () {
  let txt_json = serde_json::to_string(&config).expect("Failed to serialize config");
  let conf_name = build_config_path(env);
  let path = std::path::Path::new(conf_name.as_str());

  std::fs::write(path, txt_json).expect("Failed to write config");
}

fn read_dyn_config(env: &str) -> Value {
  let txt = match read_txt_config(env) {
    Some(txt) => txt,
    None => "{}".to_string(),
  };

  let json_obj_res = serde_json::from_str::<Value>(txt.as_str());

  return match json_obj_res {
    Ok(json) => json,
    Err(_) => serde_json::from_str::<Value>("{}").unwrap()
  };
}

fn read_txt_config(env: &str) -> Option<String> {
  let conf_name = build_config_path(env);
  let path = std::path::Path::new(conf_name.as_str());

  if !path.exists() { return None; }

  return match std::fs::read_to_string(path) {
    Ok(txt) => Some(txt),
    Err(_) => None
  };
}

fn read_safe_config(env: &str) -> AppConfig {
  let txt = read_txt_config(env).expect("Failed to read config file");
  return serde_json::from_str::<AppConfig>(txt.as_str()).expect("Failed to deserialize config");
}

fn build_config_path(env: &str) -> String {
  format!("{}.{}.json", CONFIG_FILE, env)
    .replace("..", ".")
}
