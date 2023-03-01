use docker_rust_api::apis::configuration::Configuration;
use docker_rust_api::apis::container_api::{container_create, container_start};
use docker_rust_api::apis::network_api::{network_connect};
use docker_rust_api::models::{ContainerCreateRequest, NetworkConnectRequest};
use docker_rust_api::models::HostConfig;
use uuid::Uuid;


/*

source_file: songs.csv
transit_file: songs_tranit.csv
process_id: 323-324-321-423
paralon_offset: 0
paralons:
    - [0] reduce top 10
        - container_name: <process_id>_<pipe_offset> 323-324-321-423_1
        - pull songs.csv
        - start process
        - push 323-324-321-423_1.csv
        - watch if container stopped and desired file exist cleanup current pipeline
    - [1] remove punctuation
        - container_name: <process_id>_<pipe_offset> 323-324-321-423_2
        - pull 323-324-321-423_1.csv
        - start process
        - push 323-324-321-423_2.csv
        - watch if container stopped and desired file exist cleanup current pipeline

*/

pub async fn run_pipe() {

    let config = &Configuration {
        base_path: "http://127.0.0.1:2375".to_string(),
        user_agent: None,
        client: reqwest::Client::new(),
        basic_auth: None,
        oauth_access_token: None,
        bearer_access_token: None,
        api_key: None,
    };

    let env_vars: Vec<String> = vec![
        String::from("MINIO_BUCKET_NAME=paralon"),
        String::from("MINIO_REGION=indonesia"),
        String::from("MINIO_ENDPOINT=http://paralon-minio-1:9000"),
        String::from("MINIO_ACCESS_KEY=xGLY81D0V7OBUuXT"),
        String::from("MINIO_SECRET_KEY=lBWYLLYhDyQtvUai7cpxJphDDRXniWoH"),
        String::from("MINIO_SECURITY_TOKEN="),
        String::from("MINIO_SESSION_TOKEN="),
        String::from("MINIO_PROFILE="),
        String::from("MINIO_FILE_NAME=top-songs-transit.csv"),
        String::from("MINIO_FILE_OUT_NAME=top-songs-transit.csv"),
    ];

    let pipe_executable_file: Vec<String> = vec![
        String::from("/Users/alfiankan/development/paralon/example/scripts/top_ten.py:/data/pipe.py"),
    ];

    let host_config: HostConfig = HostConfig {
        cpu_shares: None,
        memory: None,
        cgroup_parent: None,
        blkio_weight: None,
        blkio_weight_device: None,
        blkio_device_read_bps: None,
        blkio_device_write_bps: None,
        blkio_device_read_i_ops: None,
        blkio_device_write_i_ops: None,
        cpu_period: None,
        cpu_quota: None,
        cpu_realtime_period: None,
        cpu_realtime_runtime: None,
        cpuset_cpus: None,
        cpuset_mems: None,
        devices: None,
        device_cgroup_rules: None,
        device_requests: None,
        kernel_memory_tcp: None,
        memory_reservation: None,
        memory_swap: None,
        memory_swappiness: None,
        nano_cpus: None,
        oom_kill_disable: None,
        init: None,
        pids_limit: None,
        ulimits: None,
        cpu_count: None,
        cpu_percent: None,
        io_maximum_i_ops: None,
        io_maximum_bandwidth: None,
        binds: Some(pipe_executable_file),
        container_id_file: None,
        log_config: None,
        network_mode: None,
        port_bindings: None,
        restart_policy: None,
        auto_remove: None,
        volume_driver: None,
        volumes_from: None,
        mounts: None,
        console_size: None,
        cap_add: None,
        cap_drop: None,
        cgroupns_mode: None,
        dns: None,
        dns_options: None,
        dns_search: None,
        extra_hosts: None,
        group_add: None,
        ipc_mode: None,
        cgroup: None,
        links: None,
        oom_score_adj: None,
        pid_mode: None,
        privileged: None,
        publish_all_ports: None,
        readonly_rootfs: None,
        security_opt: None,
        storage_opt: None,
        tmpfs: None,
        uts_mode: None,
        userns_mode: None,
        shm_size: None,
        sysctls: None,
        runtime: None,
        isolation: None,
        masked_paths: None,
        readonly_paths: None,
    };
    
    let bind_file_pipe: Box<HostConfig> = Box::new(host_config);

    let uniq: Uuid = Uuid::new_v4();
    let process_id = uniq.to_string();

    let containers = container_create(config, ContainerCreateRequest{
        hostname: None,
        domainname: None,
        user: None,
        attach_stdin: None,
        attach_stdout: None,
        attach_stderr: None,
        exposed_ports: None,
        tty: None,
        open_stdin: None,
        stdin_once: None,
        env: Some(env_vars),
        cmd: None,
        healthcheck: None,
        args_escaped: None,
        image: Some(String::from("paralon")),
        volumes: None,
        working_dir: None,
        entrypoint: None,
        network_disabled: None,
        mac_address: None,
        on_build: None,
        labels: None,
        stop_signal: None,
        stop_timeout: None,
        shell: None,
        host_config: Some(bind_file_pipe),
        networking_config: None,
    }, Some(process_id.as_str()), None);

    let created_container = match containers.await {
        Ok(res) => res,
        Err(err) => {
            panic!("{}", err)
        }
    };



    // connect to infras network

    match network_connect(config, "4ae4ef1816b0", NetworkConnectRequest{
        container: Some(created_container.clone().id),
        endpoint_config: None,
    }).await {
        Ok(_) => {}
        Err(err) => {
            panic!("{}", err)
        }
    }

    // start container

    match container_start(config, created_container.id.as_str(), None).await {
        Ok(_) => {
            println!("{}", "container ran successfully")
        }
        Err(err) => {
            panic!("{}", err)
        }
    }


}