use fuck_sibo::prelude::*;
use fuck_sibo::{cli, submit_article};
use std::io::Write;
use tokio::task::JoinSet;
use tracing_log::{log, LogTracer};

async fn search_school_by_name(name: &str) {
    let schools = match search_school(name).await {
        Ok(x) => x,
        Err(err) => {
            log::error!("{:#?}", err);
            return;
        }
    };
    schools.into_iter().for_each(|school| {
        println!("name: {}\tID: {}", school.name, school.id);
    });
}

async fn fuck(user: &cli::User) {
    log::info!("[{}] Try login as {}", user.username, user.username);
    let user_info = match login(
        user.username.as_str(),
        user.password.as_str(),
        user.school_id.clone().unwrap().as_str(),
    )
    .await
    {
        Ok(x) => x,
        Err(err) => {
            log::error!("[{}] {}", user.username, err);
            log::trace!("[{}] {:#?}", user.username, err);
            return;
        }
    };
    log::info!("[{}] Login Successed.", user.username);
    log::trace!("[{}] {:#?}", user.username, user_info);
    log::info!("[{}] Get Classes....", user.username);
    let classes = match get_classes(&user_info.id).await {
        Ok(x) => x,
        Err(err) => {
            log::error!("[{}] {}", user.username, err);
            log::trace!("[{}] {:#?}", user.username, err);
            return;
        }
    };
    if classes.is_empty() {
        log::error!(
            "[{}] {} does not belong to any class",
            user.username,
            user.username
        );
        return;
    }
    let class = classes.first().unwrap().clone();
    log::info!("[{}] Crawing articles....", user.username);
    let articles = match get_articles(&user_info.id, &class.id, None, None).await {
        Ok(x) => x,
        Err(err) => {
            log::error!("[{}] {}", user.username, err);
            log::trace!("[{}] {:#?}", user.username, err);
            return;
        }
    };
    let mut successed_num = 0;
    let total_num = user.number_of_article.unwrap();
    for article in articles.iter() {
        if successed_num >= total_num {
            break;
        }
        log::info!("[{}] Attempting article '{}'", user.username, article.title);
        match submit_article(&user_info.id, &class.id, article, None).await {
            Ok(_) => {
                successed_num += 1;
                log::info!(
                    "[{}] Successfully submitted article({} / {}) '{}'.",
                    user.username,
                    successed_num,
                    total_num,
                    article.title,
                );
            }
            Err(err) => {
                log::warn!(
                    "[{}] Failed to submit article '{}': {}.",
                    user.username,
                    article.title,
                    err
                );
            }
        };
    }
    if successed_num < total_num {
        log::error!(
            "[{}] The number of completions is lower than the specified value({} / {})",
            user.username,
            successed_num,
            total_num,
        );
    }
}

fn main() -> std::process::ExitCode {
    let mut stderr = std::io::stderr();
    if tracing::subscriber::set_global_default(
        tracing_subscriber::fmt::Subscriber::builder()
            .with_target(false)
            .finish(),
    )
    .is_err()
    {
        let _ = writeln!(&mut stderr, "Failed to set global tracing subscriber");
        return std::process::ExitCode::FAILURE;
    };

    if LogTracer::builder().init().is_err() {
        let _ = writeln!(&mut stderr, "Failed to initalize logger");
        return std::process::ExitCode::FAILURE;
    };

    let args = match cli::parse_commandline_arguments() {
        Ok(args) => args,
        Err(e) => panic!("{}", e),
    };
    match args.command {
        cli::CommandType::Cli => {
            let thread_num: usize = args.thread_num.unwrap_or(1).try_into().unwrap();
            log::info!("Set up as {thread_num} threads");
            let rt = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(thread_num)
                .enable_all()
                .build()
                .unwrap();
            let mut task_pool = JoinSet::new();

            rt.block_on(async {
                for user in args.users.unwrap() {
                    while task_pool.len() >= thread_num {
                        task_pool.join_next().await;
                    }

                    task_pool.spawn(async move {
                        fuck(&user).await;
                    });
                }

                while !task_pool.is_empty() {
                    task_pool.join_next().await;
                }
                std::process::ExitCode::SUCCESS
            })
        }
        cli::CommandType::SearchSchool => {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(async {
                search_school_by_name(&args.school_name.unwrap()).await;
                std::process::ExitCode::SUCCESS
            })
        }
        cli::CommandType::Tui => {
            unimplemented!("暂未实现");
        }
    }
}
