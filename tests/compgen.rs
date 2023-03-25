use super::SPEC_SCRIPT;

const HELP_TAG_SCRIPT: &str = r#"
# @help Print help information

# @cmd
foo() { :; }

# @cmd
bar() { :; }
"#;

const DYNAMIC_COMPGEN_SCRIPT: &str = r#"
# @option --foo
_compgen() { :; }
"#;

#[test]
fn test_compgen() {
    snapshot_compgen!(SPEC_SCRIPT, "");
}

#[test]
fn test_compgen_help() {
    snapshot_compgen!(SPEC_SCRIPT, "help");
}

#[test]
fn test_compgen_subcommand() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_option_names ");
}

#[test]
fn test_compgen_option_choices() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_option_names --opt7 ");
}

#[test]
fn test_compgen_option_choices2() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_option_names --opt7 a");
}

#[test]
fn test_compgen_option_choices3() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_option_names --opt7 a ");
}

#[test]
fn test_compgen_positional() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_positional_requires ");
}

#[test]
fn test_compgen_positional_arg() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_positional_requires arg1 ");
}

#[test]
fn test_compgen_positional_arg2() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_positional_requires arg1 arg2 ");
}

#[test]
fn test_compgen_positional_choices() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_positional_with_choices ");
}

#[test]
fn test_compgen_help_tag() {
    snapshot_compgen!(HELP_TAG_SCRIPT, "");
}

#[test]
fn test_compgen_help_tag2() {
    snapshot_compgen!(HELP_TAG_SCRIPT, "help");
}

#[test]
fn test_compgen_choice_fn() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_option_names --op11 ");
}

#[test]
fn test_compgen_nested_command() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_nested_command ");
}

#[test]
fn test_compgen_nested_command_subcommand() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_nested_command foo ");
}

#[test]
fn test_dynamic_compgen() {
    snapshot_compgen!(DYNAMIC_COMPGEN_SCRIPT, "");
}

#[test]
fn test_compgen_multiple_notation() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_option_notations --opt2");
}

#[test]
fn test_compgen_multiple_notation2() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_option_notations --opt2 foo");
}

#[test]
fn test_compgen_multiple_notation3() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_option_notations --opt2 foo ");
}

#[test]
fn test_compgen_multiple_notation4() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_option_notations --opt2 foo bar");
}

#[test]
fn test_compgen_multiple_notation5() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_option_notations --opt2 foo bar ");
}

#[test]
fn test_compgen_unknown_option() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_omitted --unknown foo --flag1 ");
}

#[test]
fn test_compgen_unknown_option2() {
    snapshot_compgen!(SPEC_SCRIPT, "cmd_omitted --unknown foo ");
}
