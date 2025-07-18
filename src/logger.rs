use crate::app_config;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(app_config::get_server().log_level()))
        .with(
            tracing_subscriber::fmt::layer()
                .with_file(false) // 是否显示源文件目录
                .with_line_number(false) // 是否显示 log 语句在源文件的行号
                .with_thread_ids(true) // 是否显示线程 id
                .with_thread_names(true) // 是否显示线程名
                .with_target(true) // 是否显示目标文件名
                .pretty()
                .with_writer(std::io::stdout), // 写到哪里
        )
        .init();
}
