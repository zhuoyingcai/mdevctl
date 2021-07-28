use scripttemplate::*;

struct Script;

impl ScriptFunctions for Script {
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

fn main() {
    run_script_and_exit("", "", &Script);
}
