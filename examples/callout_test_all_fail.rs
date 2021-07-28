use scripttemplate::*;

struct Script;

// The post functions should never be executed if the pre callouts fail
impl ScriptFunctions for Script {
    fn pre_start(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn post_start(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn pre_define(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn post_define(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn pre_modify(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn post_modify(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn pre_stop(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn post_stop(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn pre_undefine(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn post_undefine(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn pre_list(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn post_list(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn get_attributes(&self, _opts: ScriptOpts) -> i32 {
        println!("bad json");
        1
    }
    fn notify(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn pre_test(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn post_test(&self, _opts: ScriptOpts) -> i32 {
        1
    }
}

fn main() {
    run_script_and_exit("", "", &Script);
}
