use clap::{crate_authors, crate_name, crate_version, App, SubCommand};
use constata_client_lib::{CliResult, Client, SdkResult, SubcommandResult};
use dialoguer::{theme::ColorfulTheme, Password, Select};

fn main() {
    let mut app = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about("CLI for Constata.eu's Bitcoin timestamping")
    .arg_from_usage("-c, --config=[FILE]  'Sets a custom config file'")
    .arg_from_usage("--password=[PASSWORD] 'Use this daily password. Will prompt for a password if missing.'")
    .subcommand(
      SubCommand::with_name("stamp")
        .about("Timestamps a document. Stores a copy in constata's servers.")
        .arg_from_usage("<FILE> 'Path to the file to upload and timestamp'")
     )
    .subcommand(
      SubCommand::with_name("list").about("List all your documents")
     )
    .subcommand(
      SubCommand::with_name("show")
        .about("Show a document's timestamping status.")
        .arg_from_usage("<ID> 'The document unique ID'")
     )
    .subcommand(
      SubCommand::with_name("fetch-proof")
        .about("Downloads a document's self validating HTML proof. A single HTML for all the document parts.")
        .arg_from_usage("<ID> 'The document unique id'")
     )
    .subcommand(
      SubCommand::with_name("fetch-each-proof")
        .about("Downloads a ZIP file containing one self validating HTML proof for each document part.")
        .arg_from_usage("<ID> 'The document unique id'")
     );

    let mut help = vec![];
    app.write_long_help(&mut help).unwrap();

    pub fn help_result(help: Vec<u8>) -> SubcommandResult {
        SubcommandResult::cli_ok(help)
    }

    let matches = app.get_matches();

    let config_path = matches.value_of("config");

    if Client::config_needed(config_path) {
        println!(
            "\
      Constata's API authenticates you using your own private key.\n\
      This key is never sent to our servers, and is stored encrypted in your drive.\n\
      We looked here for a config file named {} and couldn't find any.\n\
      If you already have a config file bring it over, otherwise, we can create one now.
    ",
            Client::config_path(config_path)
        );

        let items = vec![
            "Let's create one now.",
            "Exit for now. I'll bring my config over.",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("What do you want to do?")
            .items(&items)
            .default(0)
            .interact()
            .expect("Need to select an action");

        if selection == 1 {
            return println!("Ok, copy your config file here and try again.");
        } else {
            create_config_file();
        }
    }

    let daily_pass = matches
        .value_of("password")
        .map(|i| i.to_string())
        .unwrap_or_else(|| {
            Password::with_theme(&ColorfulTheme::default())
                .with_prompt("Enter your password")
                .interact()
                .unwrap()
        });
    //TODO: Handle this error, but it needs load function to add Error on the result.
    let client = Client::load(config_path, &daily_pass).unwrap();

    let result: SubcommandResult = match matches.subcommand() {
        ("stamp", Some(sub)) => {
            client.sign_and_timestamp_path(&sub.value_of("FILE").expect("File to be set"))
        }
        ("list", Some(_)) => client.documents(),
        ("show", Some(sub)) => client.document(&sub.value_of("ID").unwrap()),
        ("fetch-proof", Some(sub)) => client.fetch_proof(&sub.value_of("ID").unwrap(), true),
        ("fetch-each-proof", Some(sub)) => client.fetch_each_proof(&sub.value_of("ID").unwrap()),
        _ => help_result(help),
    };

    use std::io::Write;

    match result {
        SubcommandResult::Cli(clires) => {
            match clires {
                CliResult::Binary(res) => {
                    match res {
                        Ok(d) => std::io::stdout().write_all(&d),
                        Err(error) => std::io::stdout().write_all(&error.as_bytes().to_vec()),
                    };
                }
                CliResult::Json(res) => {
                    match res {
                        Ok(_d) => {
                            //TODO:
                        }
                        Err(_error) => {
                            //TODO:
                        }
                    };
                }
            };
        }
        SubcommandResult::Sdk(sdkres) => {
            match sdkres {
                SdkResult::Binary(_res) => {
                    //TODO:
                }
                SdkResult::Json(_res) => {
                    //TODO:
                }
            }
        }
    }
    //std::io::stdout().write_all(&result);
    println!("");
}

fn create_config_file() {
    println!("\
    You authenticate to our API by signing your requests with your own digital signature.\n\
    This tool will create a private key for you and store it locally on a file in your local drive.\n\
    You can optionally encrypt your key with a daily password, so that anyone with access to the file can't use it.\n\
    Your daily password and your decrypted key will only live in memory, and will never be stored locally.\n\
    The key is generated locally and is never sent to our servers.\n\
    This means that we can't help you if you lose it, so you should back it up.\n\
    \n\
    Along with your key, we will generate a master seed.\n\
    A master seed is composed of 12 words that you should keep safe,\n\
    and a password that you should remember.\n\
    Should your private key be compromised on its daily usage, you can use your master seed to\n\
    invalidate your compromised key.\n\
    Since you don't use your master seed on a daily basis, it shouldn't be subject to be compromised so often.\n\
    Nevertheless, you should never reveal your seed, and make sure you can remember your seed password.\n\
    We suggest you write the master seed words in paper and store them safely.\n\
  ");

    let daily_pass = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Type a daily password")
        .with_confirmation("Repeat password", "Error: the passwords don't match.")
        .interact()
        .unwrap();

    let backup_pass = Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Type a different password for the master seed")
        .with_confirmation("Repeat password", "Error: the passwords don't match.")
        .interact()
        .unwrap();

    let words = Client::create(None, None, &backup_pass, &daily_pass).unwrap();

    println!(
        "\
    A file has been created in {} with your private key\n\
    encrypted with your daily password.\n",
        Client::config_path(None)
    );

    println!(
        "\
    Now, we need you to write down these words on paper, in the order they're presented.\n\
    They are your master seed. Don't write down your master seed password though."
    );

    for (i, word) in words.iter().enumerate() {
        println!("{:02}. {}", i + 1, word);
    }
}
