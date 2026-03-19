use anyhow::{Context, Result, bail};
use std::env;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use std::process::Command;

use serde::Deserialize;
use serde::de::DeserializeOwned;

const IPC_MAGIC: [u8; 6] = *b"i3-ipc";
const IPC_SUBSCRIBE: u32 = 2;
const IPC_GET_INPUTS: u32 = 100;
const IPC_INPUT_EVENT: u32 = (1 << 31) | 21;

#[derive(Deserialize)]
struct SuccessResponse {
    success: bool,
}

#[derive(Deserialize)]
struct InputState {
    xkb_active_layout_name: Option<String>,
}

#[derive(Deserialize)]
struct InputEvent {
    input: InputState,
}

pub struct InputStream(UnixStream);

impl InputStream {
    pub fn connect() -> Result<Self> {
        let socket_path = get_socket_path()?;
        let mut stream = UnixStream::connect(&socket_path).with_context(|| {
            format!("connecting to sway IPC socket at {}", socket_path.display())
        })?;

        subscribe_to_input_events(&mut stream)?;

        Ok(Self(stream))
    }

    pub fn current_layout(&mut self) -> Result<Option<String>> {
        send_command(&mut self.0, IPC_GET_INPUTS, b"").context("sending get_inputs request")?;
        let inputs: Vec<InputState> =
            receive_json(&mut self.0, IPC_GET_INPUTS).context("reading get_inputs reply")?;
        Ok(inputs
            .into_iter()
            .find_map(|input| input.xkb_active_layout_name))
    }

    pub fn next_layout_change(&mut self) -> Result<Option<String>> {
        loop {
            let (reply_type, payload) =
                receive_message(&mut self.0).context("reading sway IPC event")?;
            if reply_type != IPC_INPUT_EVENT {
                continue;
            }

            let event: InputEvent =
                serde_json::from_slice(&payload).context("decoding input event payload")?;
            return Ok(event.input.xkb_active_layout_name);
        }
    }
}

fn get_socket_path() -> Result<PathBuf> {
    if let Ok(path) = env::var("I3SOCK").or_else(|_| env::var("SWAYSOCK")) {
        return Ok(PathBuf::from(path));
    }

    for wm in ["i3", "sway"] {
        let output = Command::new(wm).arg("--get-socketpath").output();
        if let Ok(output) = output {
            if output.status.success() {
                let path = String::from_utf8(output.stdout)?.trim().to_string();
                if !path.is_empty() {
                    return Ok(PathBuf::from(path));
                }
            }
        }
    }

    bail!("could not determine sway IPC socket path")
}

fn send_command(stream: &mut UnixStream, command_type: u32, payload: &[u8]) -> io::Result<()> {
    let mut message = Vec::with_capacity(14 + payload.len());
    message.extend_from_slice(&IPC_MAGIC);
    message.extend_from_slice(&(payload.len() as u32).to_ne_bytes());
    message.extend_from_slice(&command_type.to_ne_bytes());
    message.extend_from_slice(payload);
    stream.write_all(&message)
}

fn receive_message(stream: &mut UnixStream) -> io::Result<(u32, Vec<u8>)> {
    let mut header = [0_u8; 14];
    stream.read_exact(&mut header)?;
    if header[..6] != IPC_MAGIC {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "invalid sway IPC magic",
        ));
    }

    let mut payload_len_bytes = [0_u8; 4];
    payload_len_bytes.copy_from_slice(&header[6..10]);
    let payload_len = u32::from_ne_bytes(payload_len_bytes) as usize;

    let mut payload_type_bytes = [0_u8; 4];
    payload_type_bytes.copy_from_slice(&header[10..14]);
    let payload_type = u32::from_ne_bytes(payload_type_bytes);
    let mut payload = vec![0_u8; payload_len];
    stream.read_exact(&mut payload)?;
    Ok((payload_type, payload))
}

fn receive_json<T: DeserializeOwned>(stream: &mut UnixStream, expected_type: u32) -> Result<T> {
    let (reply_type, payload) = receive_message(stream)?;
    if reply_type != expected_type {
        bail!("unexpected reply type: expected {expected_type}, got {reply_type}");
    }

    Ok(serde_json::from_slice(&payload)?)
}

fn subscribe_to_input_events(stream: &mut UnixStream) -> Result<()> {
    send_command(stream, IPC_SUBSCRIBE, br#"["input"]"#).context("sending input subscription")?;
    let reply: SuccessResponse =
        receive_json(stream, IPC_SUBSCRIBE).context("reading subscribe reply")?;
    if !reply.success {
        bail!("sway rejected the input subscription");
    }

    Ok(())
}
