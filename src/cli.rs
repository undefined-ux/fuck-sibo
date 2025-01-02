use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use thiserror::Error;
use CommandType::Cli;

#[derive(Debug, Default)]
pub enum CommandType {
    #[default]
    Cli,
    SearchSchool,
    Tui,
}

fn read_configuration_file<P: AsRef<Path>>(path: P) -> Result<Root, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path).expect("Failed to read file");
    match toml::from_str(&contents) {
        Ok(config) => Ok(config),
        Err(e) => Err(Box::new(e)),
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    thread: u32,
    default_school_id: String,
    default_number_of_article: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub password: String,
    pub school_id: Option<String>,
    pub number_of_article: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Root {
    config: Config,
    user: Vec<User>,
}

#[derive(Debug, Default)]
pub struct Args {
    pub command: CommandType,
    pub school_name: Option<String>,
    pub users: Option<Vec<User>>,
    pub thread_num: Option<usize>,
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct CommandArgumentsParseError {
    pub message: String,
}

fn build_commandline_arguments_parser() -> Command {
    Command::new("fuck-sibo")
        .author("undefined-ux<undefined_1@outlook.com>")
        .subcommand(
            Command::new("search").about("按学校名称查询Id").arg(
                Arg::new("name")
                    .index(1)
                    .action(ArgAction::Set)
                    .value_parser(value_parser!(String)),
            ),
        )
        .subcommand(
            Command::new("fuck")
                .about("自动完成文章练习")
                .arg(
                    Arg::new("thread")
                        .short('t')
                        .action(ArgAction::Set)
                        .default_value("1")
                        .help("线程数")
                        .value_parser(value_parser!(usize)),
                )
                .arg(
                    Arg::new("username")
                        .short('u')
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    Arg::new("password")
                        .short('p')
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    Arg::new("schoolId")
                        .short('s')
                        .ignore_case(true)
                        .help("可通过search命令查找对应学校Id")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(String)),
                )
                .arg(
                    Arg::new("articleNumber")
                        .short('n')
                        .action(ArgAction::Set)
                        .ignore_case(true)
                        .help("需完成篇数")
                        .default_value("1")
                        .value_parser(value_parser!(usize)),
                ),
        )
        .subcommand(
            Command::new("ci")
                .about("按配置文件设定自动完成文章练习")
                .arg(
                    Arg::new("config")
                        .short('c')
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(String)),
                ),
        )
}

fn parse_search_subcommand(m: &ArgMatches) -> Result<Args, Box<dyn Error>> {
    Ok(Args {
        command: CommandType::SearchSchool,
        school_name: Some(m.get_one::<String>("name").unwrap().clone()),
        ..Default::default()
    })
}

fn parse_ci_subcommand(m: &ArgMatches) -> Result<Args, Box<dyn Error>> {
    let configuration_file_path = m.get_one::<String>("config").unwrap().clone();
    match read_configuration_file(configuration_file_path) {
        Ok(config) => Ok(Args {
            command: Cli,
            school_name: None,
            users: Some(
                config
                    .user
                    .into_iter()
                    .map(|it| User {
                        username: it.username,
                        password: it.password,
                        school_id: Some(
                            it.school_id
                                .unwrap_or(config.config.default_school_id.clone()),
                        ),
                        number_of_article: Some(
                            it.number_of_article
                                .unwrap_or(config.config.default_number_of_article),
                        ),
                    })
                    .collect(),
            ),
            thread_num: Some(config.config.thread as usize),
        }),
        Err(err) => Err(Box::new(CommandArgumentsParseError {
            message: format!("Could not read configuration file: {}", err),
        })),
    }
}

fn parse_fuck_subcommand(m: &ArgMatches) -> Result<Args, Box<dyn Error>> {
    if !m.contains_id("username") {
        return Err(Box::new(CommandArgumentsParseError {
            message: "Error when parse commandline arguments: missing field username".to_string(),
        }));
    } else if !m.contains_id("password") {
        return Err(Box::new(CommandArgumentsParseError {
            message: "Error when parse commandline arguments: missing field password".to_string(),
        }));
    } else if !m.contains_id("schoolId") {
        return Err(Box::new(CommandArgumentsParseError {
            message: "Error when parse commandline arguments: missing field schoolId".to_string(),
        }));
    }
    Ok(Args {
        command: Cli,
        thread_num: Some(*m.get_one::<usize>("thread").unwrap()),
        users: Some(vec![User {
            username: m.get_one::<String>("username").unwrap().clone(),
            password: m.get_one::<String>("password").unwrap().clone(),
            school_id: Some(m.get_one::<String>("schoolId").unwrap().clone()),
            number_of_article: Some(*m.get_one::<usize>("articleNumber").unwrap() as u32),
        }]),
        ..Default::default()
    })
}
pub fn parse_commandline_arguments() -> Result<Args, Box<dyn Error>> {
    let c = build_commandline_arguments_parser().get_matches();
    match c.subcommand() {
        Some(("search", m)) => parse_search_subcommand(m),
        Some(("ci", m)) => parse_ci_subcommand(m),
        Some(("fuck", m)) => parse_fuck_subcommand(m),
        _ => {
            panic!("invalid command");
        }
    }
}
