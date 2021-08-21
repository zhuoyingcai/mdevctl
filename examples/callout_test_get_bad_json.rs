use scripttemplate::*;

struct Script;

impl ScriptFunctions for Script {
    fn get_attributes(&self, _opts: ScriptOpts) -> i32 {
        println!("not json at all");
        0
    }
}

fn main() {
    run_script_and_exit("", "", &Script);
}
