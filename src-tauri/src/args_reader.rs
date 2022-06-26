pub(crate) struct AppArgs {
  pub env: String
}

pub(crate) fn read_args() -> AppArgs {
  let args: Vec<String> = std::env::args().collect();

  let mut app_args = AppArgs {
    env: "".to_string()
  };
  
  let i: usize = 1;
  let count = args.len();

  while i < count {
    let arg = &args[i];
    if arg.starts_with("--env=") {
      app_args.env = arg.replace("--env=", "");
    }
  }

  app_args
}


