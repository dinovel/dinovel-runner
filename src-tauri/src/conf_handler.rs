use serde_json::Value;

use crate::app_conf::AppConfig;

const CONFIG_FILE: &str = ".conf.json";

pub(crate) fn read_config() -> AppConfig {
  ensure_updated();

  return read_safe_config();
}

/* Ensure .conf.json is updated to latest structure */
fn ensure_updated() -> () {
  let json = read_dyn_config();
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

  match json.get("auto_port") {
    Some(v) => config.auto_port = v.as_bool().unwrap(),
    None => {}
  };

  write_config(config);
}

fn write_config(config: AppConfig) -> () {
  let txt_json = serde_json::to_string(&config).expect("Failed to serialize config");
  let path = std::path::Path::new(CONFIG_FILE);

  std::fs::write(path, txt_json).expect("Failed to write config");
}

fn read_dyn_config() -> Value {
  let txt = match read_txt_config() {
    Some(txt) => txt,
    None => "{}".to_string(),
  };

  let json_obj_res = serde_json::from_str::<Value>(txt.as_str());

  return match json_obj_res {
    Ok(json) => json,
    Err(_) => serde_json::from_str::<Value>("{}").unwrap()
  };
}

fn read_txt_config() -> Option<String> {
  let path = std::path::Path::new(CONFIG_FILE);

  if !path.exists() { return None; }

  return match std::fs::read_to_string(path) {
    Ok(txt) => Some(txt),
    Err(_) => None
  };
}

fn read_safe_config() -> AppConfig {
  let txt = read_txt_config().expect("Failed to read config file");
  return serde_json::from_str::<AppConfig>(txt.as_str()).expect("Failed to deserialize config");
}
