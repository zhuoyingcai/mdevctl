use scripttemplate::*;

struct Script;

impl ScriptFunctions for Script {}

fn main() {
    run_script_and_exit("bad_type", "bad_parent", &Script);
}
