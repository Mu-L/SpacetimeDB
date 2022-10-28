// mod logs;
mod client_api;
mod control_node_connection;
pub(crate) mod database_instance_context_controller;
mod database_logger;
mod host;
pub(crate) mod worker_budget;
mod worker_database_instance;
mod worker_db;
mod worker_metrics;

// use perf_monitor::cpu::ProcessStat;
use crate::db::db_metrics;
use tokio::spawn;

// use crate::nodes::worker_node::db_metrics::PROCESS_CPU_USAGE;

pub async fn start(config: crate::nodes::node_config::NodeConfig) {
    let worker_api_bootstrap_addr = config
        .worker_node
        .as_ref()
        .unwrap()
        .worker_api_bootstrap_addrs
        .first()
        .unwrap()
        .clone();
    let client_api_bootstrap_addr = config
        .worker_node
        .as_ref()
        .unwrap()
        .client_api_bootstrap_addrs
        .first()
        .unwrap()
        .clone();
    let advertise_addr = config.worker_node.as_ref().unwrap().advertise_addr.clone();

    // Metrics for pieces under worker_node/ related to reducer hosting, etc.
    worker_metrics::register_custom_metrics();

    // Metrics for our use of db/.
    db_metrics::register_custom_metrics();

    // spawn(async move {
    //     let mut stat = ProcessStat::cur().unwrap();
    //     loop {
    //         let usage = stat.cpu().unwrap();
    //         PROCESS_CPU_USAGE.set(usage);
    //         tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    //     }
    // });

    spawn(async move {
        control_node_connection::start(worker_api_bootstrap_addr, client_api_bootstrap_addr, advertise_addr).await;
    });

    let client_listen_addr = config.worker_node.as_ref().unwrap().listen_addr.clone();
    spawn(async move {
        client_api::start(client_listen_addr).await;
    })
    .await
    .unwrap();
}
