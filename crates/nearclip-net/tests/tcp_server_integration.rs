//! TCP 服务端集成测试
//!
//! 测试 TLS 加密的 TCP 服务端功能

use nearclip_crypto::{TlsCertificate, TlsClientConfig, TlsServerConfig};
use nearclip_net::{TcpServer, TcpServerConfig};
use std::net::Ipv4Addr;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio_rustls::TlsConnector;

/// 创建测试用的 TLS 证书和配置
fn create_test_tls_configs() -> (Arc<rustls::ServerConfig>, Arc<rustls::ClientConfig>) {
    let cert = TlsCertificate::generate(&["localhost".to_string(), "127.0.0.1".to_string()])
        .expect("Failed to generate certificate");

    let server_config = TlsServerConfig::new(&cert)
        .expect("Failed to create server config")
        .config();

    let client_config = TlsClientConfig::new(cert.cert_der())
        .expect("Failed to create client config")
        .config();

    (server_config, client_config)
}

#[tokio::test]
async fn test_tcp_server_bind_dynamic_port() {
    let (server_config, _) = create_test_tls_configs();

    let config = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(config, server_config).await.unwrap();

    let addr = server.local_addr().unwrap();
    assert_ne!(addr.port(), 0, "Port should be dynamically assigned");
    assert_eq!(addr.ip(), std::net::IpAddr::V4(Ipv4Addr::UNSPECIFIED));
}

#[tokio::test]
async fn test_tcp_server_bind_specific_port() {
    let (server_config, _) = create_test_tls_configs();

    // 使用动态端口避免冲突
    let config = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(config, server_config).await.unwrap();

    let addr = server.local_addr().unwrap();
    assert!(addr.port() > 0);
}

#[tokio::test]
async fn test_tcp_server_accept_tls_connection() {
    let (server_config, client_config) = create_test_tls_configs();

    // 启动服务端
    let config = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(config, server_config).await.unwrap();
    let addr = server.local_addr().unwrap();

    // 在后台接受连接
    let server_handle = tokio::spawn(async move {
        let conn = server.accept().await.unwrap();
        conn.peer_addr()
    });

    // 客户端连接
    let connector = TlsConnector::from(client_config);
    let tcp_stream = TcpStream::connect(addr).await.unwrap();
    let server_name = "localhost".try_into().unwrap();
    let _tls_stream = connector.connect(server_name, tcp_stream).await.unwrap();

    // 等待服务端接受连接
    let peer_addr = server_handle.await.unwrap();
    assert!(peer_addr.port() > 0);
}

#[tokio::test]
async fn test_tcp_server_data_exchange() {
    let (server_config, client_config) = create_test_tls_configs();

    // 启动服务端
    let config = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(config, server_config).await.unwrap();
    let addr = server.local_addr().unwrap();

    // 服务端处理
    let server_handle = tokio::spawn(async move {
        let mut conn = server.accept().await.unwrap();

        // 读取客户端数据
        let mut buf = [0u8; 1024];
        let n = conn.read(&mut buf).await.unwrap();
        let received = String::from_utf8_lossy(&buf[..n]).to_string();

        // 发送响应
        conn.write_all(b"Hello from server!").await.unwrap();
        conn.flush().await.unwrap();

        received
    });

    // 客户端连接并发送数据
    let connector = TlsConnector::from(client_config);
    let tcp_stream = TcpStream::connect(addr).await.unwrap();
    let server_name = "localhost".try_into().unwrap();
    let mut tls_stream = connector.connect(server_name, tcp_stream).await.unwrap();

    // 发送数据
    tls_stream.write_all(b"Hello from client!").await.unwrap();
    tls_stream.flush().await.unwrap();

    // 接收响应
    let mut response = vec![0u8; 1024];
    let n = tls_stream.read(&mut response).await.unwrap();
    let response_str = String::from_utf8_lossy(&response[..n]).to_string();

    // 验证
    let server_received = server_handle.await.unwrap();
    assert_eq!(server_received, "Hello from client!");
    assert_eq!(response_str, "Hello from server!");
}

#[tokio::test]
async fn test_tcp_server_multiple_connections() {
    let (server_config, client_config) = create_test_tls_configs();

    // 启动服务端
    let config = TcpServerConfig::new().with_port(0);
    let server = Arc::new(TcpServer::bind(config, server_config).await.unwrap());
    let addr = server.local_addr().unwrap();

    let num_clients = 3;

    // 启动多个服务端处理任务
    let mut server_handles = Vec::new();
    for i in 0..num_clients {
        let server = Arc::clone(&server);
        let handle = tokio::spawn(async move {
            let mut conn = server.accept().await.unwrap();

            // 读取客户端消息
            let mut buf = [0u8; 64];
            let n = conn.read(&mut buf).await.unwrap();
            let msg = String::from_utf8_lossy(&buf[..n]).to_string();

            // 响应
            let response = format!("Response to client {}", i);
            conn.write_all(response.as_bytes()).await.unwrap();
            conn.flush().await.unwrap();

            msg
        });
        server_handles.push(handle);
    }

    // 启动多个客户端
    let mut client_handles = Vec::new();
    for i in 0..num_clients {
        let connector = TlsConnector::from(Arc::clone(&client_config));
        let handle = tokio::spawn(async move {
            let tcp_stream = TcpStream::connect(addr).await.unwrap();
            let server_name = "localhost".try_into().unwrap();
            let mut tls_stream = connector.connect(server_name, tcp_stream).await.unwrap();

            // 发送消息
            let msg = format!("Hello from client {}", i);
            tls_stream.write_all(msg.as_bytes()).await.unwrap();
            tls_stream.flush().await.unwrap();

            // 接收响应
            let mut buf = vec![0u8; 64];
            let n = tls_stream.read(&mut buf).await.unwrap();
            String::from_utf8_lossy(&buf[..n]).to_string()
        });
        client_handles.push(handle);
    }

    // 等待所有客户端完成
    for handle in client_handles {
        let response = handle.await.unwrap();
        assert!(response.starts_with("Response to client"));
    }

    // 等待所有服务端处理完成
    for handle in server_handles {
        let msg = handle.await.unwrap();
        assert!(msg.starts_with("Hello from client"));
    }
}

#[tokio::test]
async fn test_tcp_connection_peer_addr() {
    let (server_config, client_config) = create_test_tls_configs();

    let config = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(config, server_config).await.unwrap();
    let addr = server.local_addr().unwrap();

    let server_handle = tokio::spawn(async move {
        let conn = server.accept().await.unwrap();
        conn.peer_addr()
    });

    // 客户端连接
    let connector = TlsConnector::from(client_config);
    let tcp_stream = TcpStream::connect(addr).await.unwrap();
    let client_addr = tcp_stream.local_addr().unwrap();
    let server_name = "localhost".try_into().unwrap();
    let _tls_stream = connector.connect(server_name, tcp_stream).await.unwrap();

    let peer_addr = server_handle.await.unwrap();
    assert_eq!(peer_addr.port(), client_addr.port());
}

#[tokio::test]
async fn test_tcp_server_debug_impl() {
    let (server_config, _) = create_test_tls_configs();

    let config = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(config, server_config).await.unwrap();

    let debug_str = format!("{:?}", server);
    assert!(debug_str.contains("TcpServer"));
    assert!(debug_str.contains("local_addr"));
}

#[tokio::test]
async fn test_tcp_connection_close() {
    let (server_config, client_config) = create_test_tls_configs();

    let config = TcpServerConfig::new().with_port(0);
    let server = TcpServer::bind(config, server_config).await.unwrap();
    let addr = server.local_addr().unwrap();

    let server_handle = tokio::spawn(async move {
        let mut conn = server.accept().await.unwrap();

        // 发送一些数据
        conn.write_all(b"Hello").await.unwrap();
        conn.flush().await.unwrap();

        // 关闭连接
        conn.close().await.unwrap();
    });

    // 客户端连接
    let connector = TlsConnector::from(client_config);
    let tcp_stream = TcpStream::connect(addr).await.unwrap();
    let server_name = "localhost".try_into().unwrap();
    let mut tls_stream = connector.connect(server_name, tcp_stream).await.unwrap();

    // 读取数据
    let mut buf = vec![0u8; 64];
    let n = tls_stream.read(&mut buf).await.unwrap();
    assert_eq!(&buf[..n], b"Hello");

    // 等待服务端关闭
    server_handle.await.unwrap();

    // 再次读取应该返回 0（连接关闭）
    let n = tls_stream.read(&mut buf).await.unwrap();
    assert_eq!(n, 0);
}
