//! A basic utility script used for debugging notification call-out events
//! in mdevctl. Output is sent to stdout.
//!
//! Place this script in /etc/mdevctl.d/scripts.d/notifiers/ and run
//! a supported command.

use scripttemplate::*;

struct Script;

impl ScriptFunctions for Script {
    fn notify(&self, opts: ScriptOpts) -> i32 {
        println!("logger_script: {:?}", opts);
        return 0;
    }
}

fn main() {
    run_script_and_exit("", "", &Script);
}
