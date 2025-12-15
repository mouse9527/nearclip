//! TCP 客户端集成测试
//!
//! 测试 TLS 加密的 TCP 客户端连接功能

use nearclip_crypto::{TlsCertificate, TlsClientConfig, TlsServerConfig};
use nearclip_net::{NetError, TcpClient, TcpClientConfig, TcpServer, TcpServerConfig};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

/// 创建测试用的 TLS 证书和配置
fn create_test_tls_configs() -> (
    TlsCertificate,
    Arc<rustls::ServerConfig>,
    Arc<rustls::ClientConfig>,
) {
    let cert = TlsCertificate::generate(&["localhost".to_string(), "127.0.0.1".to_string()])
        .expect("Failed to generate certificate");

    let server_config = TlsServerConfig::new(&cert)
        .expect("Failed to create server config")
        .config();

    let client_config = TlsClientConfig::new(cert.cert_der())
        .expect("Failed to create client config")
        .config();

    (cert, server_config, client_config)
}

#[tokio::test]
async fn test_tcp_client_connect_to_server() {
    let (_, server_config, client_config) = create_test_tls_configs();

    // 启动服务端
    let server_cfg = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(server_cfg, server_config).await.unwrap();
    let addr = server.local_addr().unwrap();

    // 服务端接受连接
    let server_handle = tokio::spawn(async move {
        let conn = server.accept().await.unwrap();
        conn.peer_addr()
    });

    // 客户端连接
    let client_cfg = TcpClientConfig::new(addr);
    let conn = TcpClient::connect(client_cfg, client_config, "localhost")
        .await
        .unwrap();

    assert_eq!(conn.peer_addr(), addr);

    // 等待服务端完成
    let _ = server_handle.await.unwrap();
}

#[tokio::test]
async fn test_tcp_client_data_exchange_client_to_server() {
    let (_, server_config, client_config) = create_test_tls_configs();

    // 启动服务端
    let server_cfg = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(server_cfg, server_config).await.unwrap();
    let addr = server.local_addr().unwrap();

    // 服务端接收数据
    let server_handle = tokio::spawn(async move {
        let mut conn = server.accept().await.unwrap();
        let mut buf = [0u8; 1024];
        let n = conn.read(&mut buf).await.unwrap();
        String::from_utf8_lossy(&buf[..n]).to_string()
    });

    // 客户端连接并发送数据
    let client_cfg = TcpClientConfig::new(addr);
    let mut conn = TcpClient::connect(client_cfg, client_config, "localhost")
        .await
        .unwrap();

    conn.write_all(b"Hello from client!").await.unwrap();
    conn.flush().await.unwrap();

    // 验证服务端收到的数据
    let received = server_handle.await.unwrap();
    assert_eq!(received, "Hello from client!");
}

#[tokio::test]
async fn test_tcp_client_data_exchange_server_to_client() {
    let (_, server_config, client_config) = create_test_tls_configs();

    // 启动服务端
    let server_cfg = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(server_cfg, server_config).await.unwrap();
    let addr = server.local_addr().unwrap();

    // 服务端发送数据
    let server_handle = tokio::spawn(async move {
        let mut conn = server.accept().await.unwrap();
        conn.write_all(b"Hello from server!").await.unwrap();
        conn.flush().await.unwrap();
    });

    // 客户端连接并接收数据
    let client_cfg = TcpClientConfig::new(addr);
    let mut conn = TcpClient::connect(client_cfg, client_config, "localhost")
        .await
        .unwrap();

    let mut buf = [0u8; 1024];
    let n = conn.read(&mut buf).await.unwrap();
    let received = String::from_utf8_lossy(&buf[..n]).to_string();

    assert_eq!(received, "Hello from server!");

    // 等待服务端完成
    let _ = server_handle.await.unwrap();
}

#[tokio::test]
async fn test_tcp_client_bidirectional_communication() {
    let (_, server_config, client_config) = create_test_tls_configs();

    // 启动服务端
    let server_cfg = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(server_cfg, server_config).await.unwrap();
    let addr = server.local_addr().unwrap();

    // 服务端处理
    let server_handle = tokio::spawn(async move {
        let mut conn = server.accept().await.unwrap();

        // 接收客户端消息
        let mut buf = [0u8; 1024];
        let n = conn.read(&mut buf).await.unwrap();
        let client_msg = String::from_utf8_lossy(&buf[..n]).to_string();

        // 发送响应（使用单次 write_all）
        let response = format!("Server received: {}", client_msg);
        conn.write_all(response.as_bytes()).await.unwrap();
        conn.flush().await.unwrap();

        client_msg
    });

    // 客户端连接
    let client_cfg = TcpClientConfig::new(addr);
    let mut conn = TcpClient::connect(client_cfg, client_config, "localhost")
        .await
        .unwrap();

    // 发送消息
    conn.write_all(b"ping").await.unwrap();
    conn.flush().await.unwrap();

    // 接收响应
    let mut buf = [0u8; 1024];
    let n = conn.read(&mut buf).await.unwrap();
    let response = String::from_utf8_lossy(&buf[..n]).to_string();

    assert_eq!(response, "Server received: ping");

    // 验证服务端也收到了正确的消息
    let server_received = server_handle.await.unwrap();
    assert_eq!(server_received, "ping");
}

#[tokio::test]
async fn test_tcp_client_connection_failed() {
    let (_, _, client_config) = create_test_tls_configs();

    // 尝试连接不存在的服务端
    let addr: SocketAddr = "127.0.0.1:1".parse().unwrap(); // 端口 1 通常不可用
    let client_cfg = TcpClientConfig::new(addr).with_timeout(Duration::from_millis(100));

    let result = TcpClient::connect(client_cfg, client_config, "localhost").await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    // 可能是 ConnectionFailed 或 ConnectionTimeout
    assert!(
        matches!(err, NetError::ConnectionFailed(_) | NetError::ConnectionTimeout(_)),
        "Expected ConnectionFailed or ConnectionTimeout, got {:?}",
        err
    );
}

#[tokio::test]
async fn test_tcp_client_tls_handshake_fails_with_wrong_cert() {
    let (_, server_config, _) = create_test_tls_configs();

    // 创建一个不同的证书用于客户端（不信任服务端证书）
    let wrong_cert = TlsCertificate::generate(&["wrong.example.com".to_string()])
        .expect("Failed to generate certificate");
    let wrong_client_config = TlsClientConfig::new(wrong_cert.cert_der())
        .expect("Failed to create client config")
        .config();

    // 启动服务端
    let server_cfg = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(server_cfg, server_config).await.unwrap();
    let addr = server.local_addr().unwrap();

    // 服务端等待连接（会因为握手失败而失败）
    let server_handle = tokio::spawn(async move {
        let _ = server.accept().await; // 忽略结果，客户端会断开
    });

    // 客户端尝试连接（使用错误的证书）
    let client_cfg = TcpClientConfig::new(addr);
    let result = TcpClient::connect(client_cfg, wrong_client_config, "localhost").await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(
        matches!(err, NetError::TlsHandshake(_)),
        "Expected TlsHandshake error, got {:?}",
        err
    );

    // 等待服务端完成
    let _ = server_handle.await;
}

#[tokio::test]
async fn test_tcp_client_custom_timeout() {
    // 只测试配置是否正确设置，不测试实际超时行为
    let addr: SocketAddr = "127.0.0.1:8765".parse().unwrap();
    let timeout = Duration::from_secs(5);
    let config = TcpClientConfig::new(addr).with_timeout(timeout);

    assert_eq!(config.connect_timeout, timeout);
    assert_eq!(config.target_addr, addr);
}

#[tokio::test]
async fn test_tcp_client_multiple_connections() {
    let (_, server_config, client_config) = create_test_tls_configs();

    // 启动服务端
    let server_cfg = TcpServerConfig::new().with_port(0);
    let server = Arc::new(TcpServer::bind(server_cfg, server_config).await.unwrap());
    let addr = server.local_addr().unwrap();

    let num_clients = 3;

    // 启动多个服务端处理任务
    let mut server_handles = Vec::new();
    for i in 0..num_clients {
        let server = Arc::clone(&server);
        let handle = tokio::spawn(async move {
            let mut conn = server.accept().await.unwrap();
            let mut buf = [0u8; 64];
            let n = conn.read(&mut buf).await.unwrap();
            let msg = String::from_utf8_lossy(&buf[..n]).to_string();

            let response = format!("Server {} received: {}", i, msg);
            conn.write_all(response.as_bytes()).await.unwrap();
            conn.flush().await.unwrap();

            msg
        });
        server_handles.push(handle);
    }

    // 启动多个客户端
    let mut client_handles = Vec::new();
    for i in 0..num_clients {
        let client_config = Arc::clone(&client_config);
        let handle = tokio::spawn(async move {
            let client_cfg = TcpClientConfig::new(addr);
            let mut conn = TcpClient::connect(client_cfg, client_config, "localhost")
                .await
                .unwrap();

            let msg = format!("Hello from client {}", i);
            conn.write_all(msg.as_bytes()).await.unwrap();
            conn.flush().await.unwrap();

            let mut buf = [0u8; 128];
            let n = conn.read(&mut buf).await.unwrap();
            String::from_utf8_lossy(&buf[..n]).to_string()
        });
        client_handles.push(handle);
    }

    // 等待所有客户端完成
    for handle in client_handles {
        let response = handle.await.unwrap();
        assert!(response.contains("Server"));
        assert!(response.contains("received"));
    }

    // 等待所有服务端处理完成
    for handle in server_handles {
        let msg = handle.await.unwrap();
        assert!(msg.starts_with("Hello from client"));
    }
}

#[tokio::test]
async fn test_tcp_client_connection_close() {
    let (_, server_config, client_config) = create_test_tls_configs();

    // 启动服务端
    let server_cfg = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(server_cfg, server_config).await.unwrap();
    let addr = server.local_addr().unwrap();

    // 服务端处理
    let server_handle = tokio::spawn(async move {
        let mut conn = server.accept().await.unwrap();

        // 发送数据
        conn.write_all(b"Hello").await.unwrap();
        conn.flush().await.unwrap();

        // 等待客户端关闭
        let mut buf = [0u8; 64];
        let n = conn.read(&mut buf).await.unwrap();
        n // 应该是 0，表示客户端关闭
    });

    // 客户端连接
    let client_cfg = TcpClientConfig::new(addr);
    let mut conn = TcpClient::connect(client_cfg, client_config, "localhost")
        .await
        .unwrap();

    // 读取数据
    let mut buf = [0u8; 64];
    let n = conn.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..n], b"Hello");

    // 关闭连接
    conn.close().await.unwrap();

    // 等待服务端完成
    let bytes_read = server_handle.await.unwrap();
    assert_eq!(bytes_read, 0, "Server should read 0 bytes after client close");
}

#[tokio::test]
async fn test_tcp_client_config_debug() {
    let addr: SocketAddr = "127.0.0.1:8765".parse().unwrap();
    let config = TcpClientConfig::new(addr).with_timeout(Duration::from_secs(5));

    let debug_str = format!("{:?}", config);
    assert!(debug_str.contains("TcpClientConfig"));
    assert!(debug_str.contains("127.0.0.1:8765"));
}
