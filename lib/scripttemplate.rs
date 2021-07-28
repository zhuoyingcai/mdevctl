//! Call-out script template

use std::io::{self, Read};
use structopt::StructOpt;
use strum_macros::EnumString;
use uuid::Uuid;

#[derive(EnumString, Debug)]
#[strum(ascii_case_insensitive)]
pub enum EventType {
    Pre,
    Post,
    Get,
    Notify,
}

#[derive(EnumString, Debug)]
#[strum(ascii_case_insensitive)]
pub enum CommandAction {
    Define,
    Modify,
    Start,
    Stop,
    Undefine,
    List,
    Attributes,
    Test,
}

#[derive(EnumString, Debug, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum CommandState {
    None,
    Success,
    Failure,
}

#[derive(StructOpt, Debug)]
pub struct ScriptOpts {
    #[structopt(name = "type", short, long)]
    pub mdev_type: String,
    #[structopt(short, long, help = "pre, post, notify, get")]
    event: EventType,
    #[structopt(
        short,
        long,
        help = "define, modify, start, stop, undefine, list, attributes (get only)"
    )]
    action: CommandAction,
    #[structopt(short, long, help = "success, failure, none")]
    pub state: CommandState,
    #[structopt(short, long, parse(try_from_str = Uuid::parse_str))]
    pub uuid: Uuid,
    #[structopt(short, long)]
    pub parent: String,
    #[structopt(skip)]
    pub json: String,
}

pub trait ScriptFunctions {
    fn pre_start(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn post_start(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn pre_define(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn post_define(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn pre_modify(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn post_modify(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn pre_stop(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn post_stop(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn pre_undefine(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn post_undefine(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn pre_list(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn post_list(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn get_attributes(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn notify(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn pre_test(&self, _opts: ScriptOpts) -> i32 {
        0
    }
    fn post_test(&self, _opts: ScriptOpts) -> i32 {
        0
    }
}

fn read_json_from_stdin() -> io::Result<String> {
    let mut json = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // TODO: figure out how to make this non-blocking in the
    // case where no stdin is present...
    handle.read_to_string(&mut json)?;
    Ok(json)
}

pub fn run_script_and_exit(mdev_type: &str, parent: &str, script: &impl ScriptFunctions) {
    std::process::exit(run_script(mdev_type, parent, script));
}

pub fn run_script(mdev_type: &str, parent: &str, script: &impl ScriptFunctions) -> i32 {
    let mut opts = ScriptOpts::from_args();

    if !mdev_type.is_empty() && mdev_type != opts.mdev_type {
        return 2;
    }

    if !parent.is_empty() && parent != opts.parent {
        return 2;
    }

    // Get does not expect data on stdin and
    match opts.event {
        EventType::Get => (),
        _ => {
            opts.json = read_json_from_stdin().unwrap();
        }
    }

    match opts.event {
        EventType::Get => match opts.action {
            CommandAction::Attributes => script.get_attributes(opts),
            _ => 0,
        },
        EventType::Notify => script.notify(opts),
        EventType::Pre => match opts.action {
            CommandAction::Define => script.pre_define(opts),
            CommandAction::Modify => script.pre_modify(opts),
            CommandAction::Start => script.pre_start(opts),
            CommandAction::Stop => script.pre_stop(opts),
            CommandAction::Undefine => script.pre_undefine(opts),
            CommandAction::List => script.pre_list(opts),
            CommandAction::Test => script.pre_test(opts),
            _ => 0,
        },
        EventType::Post => match opts.action {
            CommandAction::Define => script.post_define(opts),
            CommandAction::Modify => script.post_modify(opts),
            CommandAction::Start => script.post_start(opts),
            CommandAction::Stop => script.post_stop(opts),
            CommandAction::Undefine => script.post_undefine(opts),
            CommandAction::List => script.post_list(opts),
            CommandAction::Test => script.post_test(opts),
            _ => 0,
        },
    }
}
