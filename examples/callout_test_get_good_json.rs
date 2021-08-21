use scripttemplate::*;

struct Script;

impl ScriptFunctions for Script {
    fn get_attributes(&self, _opts: ScriptOpts) -> i32 {
        let data = r#"
            [
                {
                    "attribute0": "VALUE"
                }
            ]"#;
        println!("{}", data);
        0
    }
}

fn main() {
    run_script_and_exit("", "", &Script);
}
