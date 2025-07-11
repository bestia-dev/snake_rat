// automation_tasks_rs for snake_rat

// region: library and modules with basic automation tasks

mod build_cli_bin_mod;
mod build_lib_mod;
mod cargo_auto_github_api_mod;
mod encrypt_decrypt_with_ssh_key_mod;
mod generic_functions_mod;

pub use cargo_auto_lib as cl;

use crate::cargo_auto_github_api_mod as cgl;
use crate::encrypt_decrypt_with_ssh_key_mod as ende;
use crate::generic_functions_mod as gn;

pub use cl::{BLUE, GREEN, RED, RESET, YELLOW};

// traits must be in scope (Rust strangeness)
use cl::CargoTomlPublicApiMethods;

// region: library with basic automation tasks

fn main() {
    std::panic::set_hook(Box::new(gn::panic_set_hook));
    gn::tracing_init();
    cl::exit_if_not_run_in_rust_project_root_directory();
    ende::github_api_token_with_oauth2_mod::github_api_config_initialize();
    ende::crates_io_api_token_mod::crates_io_config_initialize();
    // get CLI arguments
    let mut args = std::env::args();
    // the zero argument is the name of the program
    let _arg_0 = args.next();
    match_arguments_and_call_tasks(args);
}

// region: match, help and completion

/// match arguments and call tasks functions
fn match_arguments_and_call_tasks(mut args: std::env::Args) {
    // the first argument is the user defined task: (no argument for help), build, release,...
    let arg_1 = args.next();
    match arg_1 {
        None => print_help(),
        Some(task) => {
            if &task == "completion" {
                completion();
            } else {
                println!("  {YELLOW}Running automation task: {task}{RESET}");
                if &task == "build" {
                    task_build();
                } else if &task == "release" {
                    task_release();
                } else if &task == "win_release" {
                    task_win_release();
                } else if &task == "doc" {
                    task_doc();
                } else if &task == "test" {
                    task_test();
                } else if &task == "commit_and_push" {
                    let arg_2 = args.next();
                    task_commit_and_push(arg_2);
                } else if &task == "publish_to_crates_io" {
                    task_publish_to_crates_io();
                } else if &task == "github_new_release" {
                    task_github_new_release();
                } else {
                    eprintln!("{RED}Error: Task {task} is unknown.{RESET}");
                    print_help();
                }
            }
        }
    }
}

/// write a comprehensible help for user defined tasks
fn print_help() {
    println!(
        r#"
  {YELLOW}Welcome to cargo-auto !{RESET}
  {YELLOW}This program automates your custom tasks when developing a Rust project.{RESET}

  {YELLOW}User defined tasks in automation_tasks_rs:{RESET}
{GREEN}cargo auto build{RESET} - {YELLOW}builds the crate in debug mode, fmt, increment version{RESET}
{GREEN}cargo auto release{RESET} - {YELLOW}builds the crate in release mode, fmt, increment version{RESET}
{GREEN}cargo auto win_release{RESET} - {YELLOW}builds the crate in release mode, for windows{RESET}
{GREEN}cargo auto doc{RESET} - {YELLOW}builds the docs, copy to docs directory{RESET}
{GREEN}cargo auto test{RESET} - {YELLOW}runs all the tests{RESET}
{GREEN}cargo auto commit_and_push "message"{RESET} - {YELLOW}commits with message and push with mandatory message{RESET}
  {YELLOW}It is preferred to use SSH for git push to GitHub.{RESET}
  {YELLOW}<https://github.com/CRUSTDE-ContainerizedRustDevEnv/crustde_cnt_img_pod/blob/main/ssh_easy.md>{YELLOW}
  {YELLOW}On the very first commit, this task will initialize a new local git repository and create a remote GitHub repo.{RESET}
  {YELLOW}For the GitHub API the task needs the Access secret token from OAuth2 device workflow.{RESET}
  {YELLOW}The secret token will be stored in a file encrypted with your SSH private key.{RESET}
  {YELLOW}You can type the passphrase of the private key for every usee. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET} - {YELLOW}publish to crates.io, git tag{RESET}
  {YELLOW}You need the API secret_token for publishing. Get the secret_token on <https://crates.io/settings/tokens>.{RESET}
  {YELLOW}The secret token will be stored in a file encrypted with your SSH private key.{RESET}
  {YELLOW}You can type the passphrase of the private key for every usee. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto github_new_release{RESET} - {YELLOW}creates new release on GitHub{RESET}
  {YELLOW}For the GitHub API the task needs the Access secret token from OAuth2 device workflow.{RESET}
  {YELLOW}The secret token will be stored in a file encrypted with your SSH private key.{RESET}
  {YELLOW}You can type the passphrase of the private key for every usee. This is pretty secure.{RESET}
  {YELLOW}Somewhat less secure (but more comfortable) way is to store the private key in ssh-agent.{RESET}
{GREEN}cargo auto update_automation_tasks_rs{RESET} - {YELLOW}updates the files in automation_tasks_rs{RESET}
  {YELLOW}Some files are fixed and the update is straight forward, other files need manual diff.{RESET}

  {YELLOW}© 2025 bestia.dev  MIT License github.com/automation-tasks-rs/cargo-auto{RESET}
"#
    );
    print_examples_cmd();
}

/// all example commands in one place
fn print_examples_cmd() {
    /*
            println!(
                r#"
      {YELLOW}run examples:{RESET}
    {GREEN}cargo run --example plantuml1{RESET}
        "#
            );
        */
}

/// Sub-command for bash auto-completion of `cargo auto` using the crate `dev_bestia_cargo_completion`.
fn completion() {
    let args: Vec<String> = std::env::args().collect();
    let word_being_completed = args[2].as_str();
    let last_word = args[3].as_str();

    if last_word == "cargo-auto" || last_word == "auto" {
        let sub_commands = vec![
            "build",
            "release",
            "win_release",
            "doc",
            "test",
            "commit_and_push",
            "publish_to_crates_io",
            "github_new_release",
            "update_automation_tasks_rs"
        ];
        cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    /*
    // the second level if needed
    else if last_word == "new" {
        let sub_commands = vec!["x"];
       cl::completion_return_one_or_more_sub_commands(sub_commands, word_being_completed);
    }
    */
}

// endregion: match, help and completion

// region: tasks

/// cargo build
fn task_build() {
    let cargo_toml = crate::build_cli_bin_mod::task_build();
    println!(
        r#"
  {YELLOW}After `cargo auto build`, run the compiled binary, examples and/or tests{RESET}
{GREEN}./target/debug/{package_name} {RESET}
  {YELLOW}If ok then{RESET}
{GREEN}cargo auto release{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo build --release
fn task_release() {
    let cargo_toml = crate::build_cli_bin_mod::task_release();

    println!(
        r#"
  {YELLOW}After `cargo auto release`, run the compiled binary, examples and/or tests{RESET}
{GREEN}./target/release/{package_name} {RESET}
{GREEN}cargo auto win_release{RESET}
  {YELLOW}If ok then{RESET}
{GREEN}cargo auto doc{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo doc, then copies to /docs/ folder, because this is a GitHub standard folder
fn task_doc() {
    gn::task_doc();
    // message to help user with next move
    println!(
        r#"
  {YELLOW}If ok then run the tests in code and the documentation code examples.{RESET}
{GREEN}cargo auto test{RESET}
"#
    );
}

/// cargo build --release --target x86_64-pc-windows-gnu
/// TODO: try cross compile to windows, because Linux has problems with file datetimes on external disk
fn task_win_release() {
    let cargo_toml = cl::CargoToml::read();
    cl::auto_version_increment_semver_or_date();
    cl::auto_cargo_toml_to_md();
    cl::auto_lines_of_code("");

    cl::run_shell_command_static("cargo fmt").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("cargo clippy --target x86_64-pc-windows-gnu").unwrap_or_else(|e| panic!("{e}"));
    cl::run_shell_command_static("cargo build --release --target x86_64-pc-windows-gnu").unwrap_or_else(|e| panic!("{e}"));

    // cl::ShellCommandLimitedDoubleQuotesSanitizer::new(r#"strip "target/release/{package_name}" "#)
    //     .unwrap_or_else(|e| panic!("{e}"))
    //     .arg("{package_name}", &cargo_toml.package_name())
    //     .unwrap_or_else(|e| panic!("{e}"))
    //     .run()
    //     .unwrap_or_else(|e| panic!("{e}"));

    println!(
        r#"
    {YELLOW}After `cargo auto win_release`, run the compiled binary, examples and/or tests{RESET}

    {YELLOW}In Windows git-bash, copy the exe file from the Crustde container to Windows.{RESET}
{GREEN}
mkdir -p ~/git-bash/rustprojects/{package_name} 
cd ~/git-bash/rustprojects/{package_name}
scp rustdevuser@crustde:/home/rustdevuser/rustprojects/{package_name}/target/x86_64-pc-windows-gnu/release/{package_name}.exe /c/Users/Luciano/git-bash/rustprojects/{package_name}/{RESET}
    {YELLOW}Run the exe in Windows git-bash.{RESET}
{GREEN}
./{package_name}.exe{RESET}

    {YELLOW}if ok then{RESET}
{GREEN}cargo auto doc{RESET}
"#,
        package_name = cargo_toml.package_name(),
    );
    print_examples_cmd();
}

/// cargo test
fn task_test() {
    cl::run_shell_command_static("cargo test").unwrap_or_else(|e| panic!("{e}"));
    println!(
        r#"
  {YELLOW}After `cargo auto test`. If ok then {RESET}
  {YELLOW}(commit message is mandatory){RESET}
{GREEN}cargo auto commit_and_push "message"{RESET}
"#
    );
}

/// commit and push
fn task_commit_and_push(arg_2: Option<String>) {
    gn::task_commit_and_push(arg_2);
    println!(
        r#"
  {YELLOW}After `cargo auto commit_and_push "message"`{RESET}
{GREEN}cargo auto publish_to_crates_io{RESET}
"#
    );
}

/// publish to crates.io and git tag
fn task_publish_to_crates_io() {
    let tag_name_version = crate::build_cli_bin_mod::task_publish_to_crates_io();

    println!(
        r#"
  {YELLOW}Now, write the content of the release in the RELEASES.md in the `## Unreleased` section, then{RESET}
  {YELLOW}Next, create the GitHub Release {tag_name_version}.{RESET}
{GREEN}cargo auto github_new_release{RESET}
"#
    );
}

/// create a new release on github
fn task_github_new_release() {
    gn::task_github_new_release();
    println!(r#"  {YELLOW}No more automation tasks. {RESET}"#);
}
// endregion: tasks
