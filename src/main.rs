#[macro_use]
extern crate clap;
extern crate colored;
extern crate ct;
extern crate serde_json;
extern crate linked_hash_map;

use clap::App;
use clap::Arg;
use clap::SubCommand;
use colored::*;
use ct::cli::Config;
use ct::extract::RunCommand;
use ct::file_finder::CTFile;
use ct::*;
use std::env;
use std::string::String;
use linked_hash_map::LinkedHashMap;
use ct::log::debug_log;
use ct::ports::CTPorts;


fn main() -> Result<(), String> {
    show_banner();
    let app_args: Vec<String> = env::args().collect();
    debug_log(|| String::from("Read ct file"));
    let ct_file = CTFile::get_content().ok();

    debug_log(|| String::from("Read config"));
    let config = Config::new(app_args).ok();
    debug_log(|| String::from("Running"));
    run(ct_file, config)
}


fn help(ct_file: &Option<CTFile>) -> String{
    let ports_available = CTPorts::available();
    let mut help : Vec<String> = Vec::new();
    help.push("Default commands :".green().to_string());
    if ports_available {
        help.push(format!("\t• {} runs a server on http://localhost:1500 to see other used ports 👂", "ports".blue()));
    }
    help.push(format!("\t• {} provide manual from content {{name}} of README.md 📖", "man {name}".blue()));
    help.push(String::from(""));
    if let Some(file) = ct_file {
        help.push(format!("Declared aliases found in {} :", file.path).green().to_string());
        for (alias, command) in RunCommand::all(&file.content, &None) {
            help.push(format!("\t• {} runs {} {} {}", alias.blue(), command.command.green(), command.args.join(" ").green(), command.doc.red()));
        }
    }else{
        help.push(format!("{}", "No .ctproject found in the current directory.".red()));
        help.push(format!("\t{}", "🚀 You can use ct --init to create a sample 🖖".yellow()));
    }
    help.join("\n")
}

fn run(ct_file: Option<CTFile>, config: Option<Config>) -> Result<(), String>{
    let args: Vec<App> = vec![SubCommand::with_name("man")
                                  .about("provide manual from content {{name}} of README.md 📖")
                                  .arg(Arg::with_name("help").short("h").long("help").help("Lists available topics"))
                                  .arg(Arg::with_name("name").multiple(true).help("extract content")),
                              SubCommand::with_name("ports")
                                  .about("runs a server on http://localhost:1500 to see other used ports 👂")];
    let all_commands = match ct_file {
        Some(ref ct_file) => RunCommand::all(&ct_file.content, &config),
        None => LinkedHashMap::new()
    };
    let commands_from_ctproject: Vec<App> = all_commands.iter()
        .map(|a| {
            SubCommand::with_name(&a.0).about(a.1.doc.as_str())
                .arg(Arg::with_name("ARGS").help("Additional args to pass to alias").multiple(true))
        }).collect();
    let help_text = help(&ct_file);
    let app = App::new("ct - CLI helper")
        .version(crate_version!())
        .author(crate_authors!())
        .help(help_text.as_str())
        .about("Help you to handle your project easily")
        .arg(Arg::with_name("init").long("init").help("Initialize a new project in the current directory"))
        .subcommands(args.clone())
        .subcommands(commands_from_ctproject)
        ;
    let matches = app.clone().get_matches();
    debug_log(|| format!("{:?}", matches));
    if matches.is_present("init"){
        debug_log(|| String::from("Init new project"));
        init_project();
        return Ok(())
    }
    if let Some(command) = matches.subcommand {
        if args.iter().map(|a| a.get_name().to_string()).filter(|c| c == &command.name).count() > 0{
            match command.name.as_ref() {
                "ports" => start_port_listening(),
                "man" => show_man(command.matches.value_of("name"), command.matches.is_present("help"), ct_file),
                _ => debug_log(|| String::from("")),
            }

            return Ok(());
        }
        if let Some(ct_file) = ct_file {
            let command = all_commands.get(&command.name);
            match command {
                Some(run_command) => run_command.run(&ct_file),
                None => {
                    let _ = app.clone().print_help();
                }
            }
        }
    }else{
        let _ = app.clone().print_help();
    }

    Ok(())
}



