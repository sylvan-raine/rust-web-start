use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::app_config;

pub fn init() {
    tracing_subscriber::registry()
        .with(EnvFilter::new(app_config::get_server().log_level()))
        .with(
            tracing_subscriber::fmt::layer()
                // .with_file(true)                // 是否显示源文件目录
                // .with_line_number(true)         // 是否显示 log 语句在源文件的行号
                // .with_thread_ids(true)          // 是否显示线程 id
                .with_thread_names(true)        // 是否显示线程名
                // .with_target(false)              // 是否显示目标文件名
                .with_writer(std::io::stderr)   // 写到哪里
        )
        .init();
}