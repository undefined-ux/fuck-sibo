use std::error::Error;
use tokio::task::JoinSet;
use fuck_sibo::cli;
use fuck_sibo::prelude::*;
use tracing_log::{log, LogTracer};
async fn search_school_by_name(name: &str) -> Result<(), Box<dyn Error>> {
    let schools = search_school(name).await?;
    schools.into_iter().for_each(|school| {
        println!("name: {}\nID: {}\n============================", school.name, school.id);
    });
    Ok(())
}

async fn fuck(user: &cli::User) -> Result<(), Box<dyn Error>> {
    todo!()
}


fn main() -> std::process::ExitCode {
    let mut stderr = std::io::stderr();
    match tracing::subscriber
    ::set_global_default(
        tracing_subscriber::fmt::Subscriber::builder().with_target(false).finish()
    ) {
        Err(_) => {
            let _ = writeln!(&mut stderr, "Failed to set global tracing subscriber");
            return std::process::ExitCode::FAILURE;
        },
        _ => {}
    };

    match LogTracer::builder().init() {
        Err(_) => {
            let _ = writeln!(&mut stderr, "Failed to initalize logger");
            return std::process::ExitCode::FAILURE;
        },
        _ => {}
    };


    let args = match cli::parse_commandline_arguments() {
        Ok(args) => args,
        Err(e) => panic!("{}", e)
    };
    let res = match args.command {
        cli::CommandType::Cli => {
            let thread_num: usize = args.thread_num.unwrap_or(1).try_into().unwrap();
            log::info!("Set up as {thread_num} threads");
            let rt = tokio::runtime::Builder
            ::new_multi_thread()
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
                        match fuck(&user).await {
                            Err(e) => {
                                let _ = writeln!(&mut stderr, "{}", e);
                            },
                            _ => {}
                        };
                    });
                }

                while task_pool.len() > 0 {
                    task_pool.join_next().await;
                }

            });
        },
        cli::CommandType::SearchSchool => {
            let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
            rt.block_on(async {
                match search_school_by_name(&args.school_name.unwrap()).await {
                    Err(e) => {
                        let _ = writeln!(&mut stderr, "{}", e);
                    },
                    _ => {}
                }
            });
            
        },
        cli::CommandType::Tui => {
            unimplemented!("暂未实现")

        }
    };

    match res {
        Err(_) => std::process::ExitCode::FAILURE,
        _ => std::process::ExitCode::SUCCESS
    }

}

