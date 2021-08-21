use scripttemplate::*;

struct Script;

impl ScriptFunctions for Script {
    fn pre_test(&self, _opts: ScriptOpts) -> i32 {
        1
    }
    fn post_test(&self, _opts: ScriptOpts) -> i32 {
        1
    }
}

fn main() {
    run_script_and_exit("type_c", "parent_c", &Script);
}
