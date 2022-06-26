use std::{collections::HashMap};

use tauri::{Context, Assets, api::cli::{get_matches, ArgData}};

pub(crate) struct TauriArgs {
  pub env: String
}

pub(crate) fn read_args<T: Assets>(context: &Context<T>) -> TauriArgs {
  let args = read_tauri_args(&context);

  match args {
      Some(a) => build_from_args(&a),
      None => build_default()
  }
}

fn read_tauri_args<T: Assets>(context: &Context<T>) -> Option<HashMap<String, ArgData>> {
  let config = context.config().tauri.cli.clone().unwrap();

  match get_matches(&config, context.package_info()) {
      Ok(e) => Some(e.args),
      Err(_) => None
  }
}

fn build_default() -> TauriArgs {
  TauriArgs {
    env: "".to_string()
  }
}

fn build_from_args(args: &HashMap<String, ArgData>) -> TauriArgs {

  args.iter().for_each(|e| {
    println!("{:?}", e);
  });

  TauriArgs {
    env: read_match_env(args.get("env"))
  }
}

fn read_match_env(data: Option<&ArgData>) -> String {
  match data {
      Some(e) => {
        if e.value.is_string() {
          e.value.as_str().unwrap().to_string()
        } else {
          "".to_string()
        }
      },
      None => "".to_string()
  }
}
