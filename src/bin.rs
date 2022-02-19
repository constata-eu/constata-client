use clap::{crate_authors, crate_name, crate_version, App, SubCommand};
use constata_client_lib::Client;
use dialoguer::{theme::ColorfulTheme, Password, Select};

fn main() {
  let mut app = App::new(crate_name!())
    .version(crate_version!())
    .author(crate_authors!())
    .about("CLI for Constata.eu's Bitcoin timestamping")
    .arg_from_usage("-c, --config=[FILE]  'Sets a custom config file'")
    .arg_from_usage("--password=[PASSWORD] 'Use this daily password. Will prompt for a password if missing.'")
    .subcommand(
      SubCommand::with_name("api").about("direct call and response from constata API")
      .subcommand(
        SubCommand::with_name("stamp")
          .about("Timestamps a document. Stores a copy in constata's servers.")
          .arg_from_usage("<FILE> 'Path to the file to upload and timestamp'")
      )
      .subcommand(
        SubCommand::with_name("list").about("List all your documents")
      )
    )
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
     )
    .subcommand(
      SubCommand::with_name("verify-website")
        .about("\
          Starts the process of verifying that your public key manages the given website.\
          Yo must be able to create a file called 'constata_eu_domain_verification.txt' at the website's root level.\
        ")
        .arg_from_usage("<URL> 'Your website root URL, must be https (https://example.com)'")
    )
    .subcommand(
      SubCommand::with_name("website-verifications").about("Shows the status of your website verification")
    )
    .subcommand(
      SubCommand::with_name("account-state")
      .about("Show person's account state including token balance and documents pending to be stamped")
    );

  let mut help = vec![];
  app.write_long_help(&mut help).unwrap();

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

  let client = Client::load(config_path, &daily_pass).unwrap();

  let result = match matches.subcommand() {
    ("api", Some(sub)) => {
      if let Some(stamp) = sub.subcommand_matches("stamp") {
        client
          .sign_and_timestamp_path(&stamp.value_of("FILE").expect("FILE to be set"), true)
          .expect("Sign and timestamp to succeed")
          .as_bytes()
          .to_vec()
      } else if sub.is_present("list") {
        client.documents().unwrap().as_bytes().to_vec()
      } else {
        help
      }
    },
    ("stamp", Some(sub)) => client
      .sign_and_timestamp_path(&sub.value_of("FILE").expect("FILE to be set"), false)
      .expect("Sign and timestamp to succeed")
      .as_bytes()
      .to_vec(),
    ("list", Some(_)) => client.list_documents().unwrap().as_bytes().to_vec(),
    ("show", Some(sub)) => client
      .document(&sub.value_of("ID").unwrap())
      .unwrap()
      .as_bytes()
      .to_vec(),
    ("fetch-proof", Some(sub)) => client
      .fetch_proof(&sub.value_of("ID").unwrap())
      .unwrap()
      .as_bytes()
      .to_vec(),
    ("fetch-each-proof", Some(sub)) => client
      .fetch_each_proof(&sub.value_of("ID").unwrap())
      .unwrap(),
    ("verify-website", Some(sub)) =>
      verify_website_flow(&client, &sub.value_of("URL").expect("URL TO BE SET"))
        .as_bytes()
        .to_vec(),
    ("website-verifications", Some(_)) => client.website_verifications().unwrap().as_bytes().to_vec(),
    ("account-state", Some(_)) => client
      .account_state().unwrap().as_bytes().to_vec(),
    _ => help,
  };

  use std::io::Write;
  std::io::stdout().write_all(&result).unwrap();
  println!("");
}

fn verify_website_flow(client: &Client, website: &str) -> String {
  let (_response, signature) = client
    .verify_website(website.as_bytes())
    .expect("Verify website to succeed");

  format!("\
    We have started the validation process for {}.\n\
    To verify you manage {} we need you to create a file at:\n\
    {}/constata_eu_domain_verification.txt\n\
    The file contents should be:\n\
    {}
  ", website, website, website, signature)
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
