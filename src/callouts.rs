use crate::environment::Environment;
use anyhow::{anyhow, Context, Result};
use log::debug;
use std::fmt::{self, Display, Formatter};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use crate::mdev::*;

#[derive(Clone, Copy)]
pub enum EventType {
    Pre,
    Post,
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            EventType::Pre => {
                write!(f, "pre")
            }
            EventType::Post => {
                write!(f, "post")
            }
        }
    }
}

fn match_event_dir(event: EventType, env: &dyn Environment) -> PathBuf {
    match event {
        EventType::Pre | EventType::Post => env.callout_script_base(),
    }
}

pub struct Callout<'a> {
    state: &'a str,
    conf: Option<String>,
    script: Option<PathBuf>,
    output: Option<Output>,
}

impl<'a> Callout<'a> {
    pub fn new() -> Callout<'a> {
        Callout {
            state: "none",
            conf: None,
            script: None,
            output: None,
        }
    }

    pub fn set_state(&mut self, state: &'a str) {
        self.state = state;
    }

    fn conf(&self, dev: &mut MDev) -> Result<&String> {
        self.conf.as_ref().ok_or_else(|| {
            anyhow!(
                "Failed to get {} device config file as stdin for callout script",
                dev.uuid.to_hyphenated().to_string()
            )
        })
    }

    fn script(&self) -> Result<&PathBuf> {
        self.script
            .as_ref()
            .ok_or_else(|| anyhow!("Failed to get callout script path"))
    }

    fn output(&self) -> Result<&Output> {
        self.output
            .as_ref()
            .ok_or_else(|| anyhow!("Failed to get output from callout script"))
    }

    fn invoke_script<P: AsRef<Path>>(
        &self,
        dev: &mut MDev,
        script: P,
        event: EventType,
        action: &str,
    ) -> Result<Output> {
        let e = event.to_string();

        debug!(
            "{} callout: executing {:?}",
            &e,
            script.as_ref().as_os_str()
        );

        let mut child = Command::new(script.as_ref().as_os_str())
            .arg("-t")
            .arg(dev.mdev_type()?)
            .arg("-e")
            .arg(&e)
            .arg("-a")
            .arg(action)
            .arg("-s")
            .arg(&self.state)
            .arg("-u")
            .arg(dev.uuid.to_string())
            .arg("-p")
            .arg(dev.parent()?)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        if let Some(mut child_stdin) = child.stdin.take() {
            child_stdin
                .write_all(self.conf(dev)?.as_bytes())
                .with_context(|| "Failed to write to stdin of command")?;
        }

        let output = child.wait_with_output()?;

        Ok(output)
    }

    fn print_output<P: AsRef<Path>>(&self, script: P, stderr: bool, stdout: bool) -> Result<()> {
        let sname = script
            .as_ref()
            .file_name()
            .expect("Failed to get script name")
            .to_string_lossy();

        if stderr {
            let st = String::from_utf8_lossy(&self.output()?.stderr);
            if !st.is_empty() {
                eprint!("{}: {}", &sname, st);
            }
        }
        if stdout {
            let st = String::from_utf8_lossy(&self.output()?.stdout);
            if !st.is_empty() {
                print!("{}: {}", &sname, st);
            }
        }

        Ok(())
    }

    fn pre_post(
        &mut self,
        dev: &mut MDev,
        dir: PathBuf,
        event: EventType,
        action: &str,
    ) -> Result<()> {
        let mut rc = Some(0);

        if let Some(ref s) = self.script {
            self.output = Some(self.invoke_script(dev, s, event, action)?);
            self.print_output(s, true, false)?;
            rc = self.output()?.status.code();
        } else {
            for s in dir.read_dir()? {
                let path = s?.path();

                self.output = Some(self.invoke_script(dev, &path, event, action)?);

                let tmp_rc = self.output()?.status.code();
                if tmp_rc != Some(2) {
                    rc = tmp_rc;
                    self.script = Some(path);
                    self.print_output(self.script()?, true, false)?;
                    break;
                } else {
                    debug!(
                        "Device type {} unmatched by callout script",
                        dev.mdev_type.as_ref().unwrap(),
                    );
                }
            }
        }

        match rc {
            Some(0) => Ok(()),
            _ => Err(anyhow!(
                "aborting command due to results from callout script {:?}",
                self.script()?
            )),
        }
    }

    pub fn callout(
        &mut self,
        dev: &mut MDev,
        env: &dyn Environment,
        event: EventType,
        action: &str,
    ) -> Result<()> {
        let dir = match_event_dir(event, env);

        if dir.read_dir()?.count() == 0 {
            return Ok(());
        }

        if self.conf.is_none() {
            self.conf = Some(dev.to_json(false)?.to_string());
        }

        match event {
            EventType::Pre | EventType::Post => self.pre_post(dev, dir, event, action),
        }
    }
}
