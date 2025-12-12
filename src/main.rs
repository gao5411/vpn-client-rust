use std::time::Duration;
use tokio::time::interval;

#[derive(Debug, serde::Deserialize)]
struct NodeConfig {
    node_id: String,
    manage_server: String,
    secret_key: String,
    wan_interfaces: Vec<String>,
    wg_private_key: String,
    wg_listen_port: u16,
}

#[derive(Debug, serde::Serialize)]
struct WANIPInfo {
    wan_id: String,
    public_ip: String,
    map_port: u16,
    nat_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // 加载配置
    let config_content = match std::fs::read_to_string("inner_node.toml") {
        Ok(content) => content,
        Err(_) => {
            log::error!("找不到配置文件 inner_node.toml，请确保文件存在");
            return Ok(());
        }
    };
    
    let config: NodeConfig = match toml::from_str(&config_content) {
        Ok(cfg) => cfg,
        Err(e) => {
            log::error!("配置文件解析失败: {}", e);
            return Ok(());
        }
    };

    log::info!("节点ID: {}", config.node_id);

    // 启动 WireGuard 服务
    let wg = start_wireguard(&config).await?;
    log::info!("WireGuard 监听端口: {}", config.wg_listen_port);

    // 启动 IP 上报任务
    let config_clone_report = unsafe { std::mem::transmute::<&NodeConfig, &NodeConfig>(&config) }; // 注意：实际生产应使用 Arc<NodeConfig>
    tokio::spawn(async move {
        let mut ticker = interval(Duration::from_secs(60));
        loop {
            ticker.tick().await;
            if let Err(e) = report_wan_ips(config_clone_report).await {
                log::error!("WAN 上报失败: {}", e);
            } else {
                log::info!("WAN 上报成功");
            }
        }
    });

    // 启动打洞监听
    let config_clone_punch = unsafe { std::mem::transmute::<&NodeConfig, &NodeConfig>(&config) }; // 注意：实际生产应使用 Arc<NodeConfig>
    tokio::spawn(async move {
        if let Err(e) = start_punch_listener(config_clone_punch).await {
            log::error!("打洞监听失败: {}", e);
        }
    });

    // 等待退出信号
    tokio::signal::ctrl_c().await?;
    log::info!("收到退出信号");
    wg.down()?;
    Ok(())
}

async fn start_wireguard(config: &NodeConfig) -> Result<wireguard_rs::WgDevice, Box<dyn std::error::Error>> {
    let mut wg = wireguard_rs::WgDevice::new("wg0")?;
    let cfg = wireguard_rs::WgConfig {
        private_key: config.wg_private_key.clone(),
        public_key: wireguard_rs::generate_public_key(&config.wg_private_key)?,
        listen_port: config.wg_listen_port,
        peers: vec![],
        allowed_ips: "192.168.1.0/24".to_string(),
        ..Default::default()
    };
    wg.configure(&cfg)?;
    wg.up()?;
    configure_nat()?;
    Ok(wg)
}

fn configure_nat() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("iptables")
            .args(["-t", "nat", "-A", "POSTROUTING", "-o", "eth0", "-j", "MASQUERADE"])
            .status()?;
        std::fs::write("/proc/sys/net/ipv4/ip_forward", "1")?;
    }
    Ok(())
}

async fn report_wan_ips(config: &NodeConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut list = Vec::new();
    for (idx, _) in config.wan_interfaces.iter().enumerate() {
        let public_ip = public_ip::addr(public_ip::V4).await?.to_string();
        let nat_type = "cone".to_string(); // 实际应使用 STUN 查询
        list.push(WANIPInfo { wan_id: format!("WAN{}", idx + 1), public_ip, map_port: config.wg_listen_port, nat_type });
    }

    let client = reqwest::Client::new();
    let resp = client.post(format!("{}/api/ip/update", config.manage_server))
        .json(&serde_json::json!({
            "node_id": config.node_id,
            "secret_key": config.secret_key,
            "wan_ips": list
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        return Err(format!("上报失败: {}", resp.status()).into());
    }
    Ok(())
}

async fn start_punch_listener(config: &NodeConfig) -> Result<(), Box<dyn std::error::Error>> {
    let socket = tokio::net::UdpSocket::bind(format!("0.0.0.0:{}", config.wg_listen_port + 1000)).await?;
    log::info!("UDP 打洞监听端口: {}", config.wg_listen_port + 1000);
    let mut buf = [0; 1024];
    loop {
        let (len, src) = socket.recv_from(&mut buf).await?;
        let data = String::from_utf8_lossy(&buf[..len]);
        log::info!("收到打洞指令: {} 来自: {}", data, src);
        let resp = format!("PUNCH_RESP:{}:{}", config.node_id, config.wg_listen_port);
        socket.send_to(resp.as_bytes(), &src).await?;
    }
}
