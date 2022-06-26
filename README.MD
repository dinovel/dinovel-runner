# Dinovel runner

Runs a Tauri window for a backend. Supports self starting a server or can connect to a running server.

## Configuration

### Example

```JSON
{
  "title": "My app",
  "width": 800,
  "height": 600,
  "fullscreen": false,
  "mode": "Process",
  "target": "server.exe"
}
```

### Title

    Window title.

### width

    Window width when not in fullscreen

### height

    Window height when not in fullscreen

### Fullscreen

    Start in fullscreen mode.

### Mode

    - Process
      - Starts a new process using the URI set in __Target__.
      - An argumment will be passed to the process for the target port to listen to.
        > Argument format: `--port {port}`
      - Navigates to `http://localhost:{port}`.

    - Url
      - Navigates to the URL set in __Target__.

### Target

    App path or URL to use when starting the app.

## Environments

    By default the used config file is `.dnr.json`, but if the argument `--env={name}` is used, 
    the config file will be `.dnr.{name}.json`.
