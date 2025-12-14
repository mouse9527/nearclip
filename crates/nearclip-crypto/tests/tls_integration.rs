//! TLS 集成测试 - 验证完整的 TLS 1.3 握手流程
//!
//! 这些测试验证 AC5: "集成测试验证加密连接建立"

use nearclip_crypto::{TlsCertificate, TlsClientConfig, TlsServerConfig};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{TlsAcceptor, TlsConnector};

/// 测试完整的 TLS 1.3 握手和数据交换
///
/// 验证：
/// 1. 服务端可以使用生成的证书接受连接
/// 2. 客户端可以使用 TOFU 信任模型连接
/// 3. 双方可以通过加密通道交换数据
#[tokio::test]
async fn test_tls_handshake_and_data_exchange() {
    // 生成服务端证书
    let server_cert = TlsCertificate::generate(&["localhost".to_string()])
        .expect("Failed to generate certificate");

    // 创建服务端和客户端配置
    let server_config = TlsServerConfig::new(&server_cert)
        .expect("Failed to create server config");
    let client_config = TlsClientConfig::new(server_cert.cert_der())
        .expect("Failed to create client config");

    // 创建 TLS acceptor 和 connector
    let acceptor = TlsAcceptor::from(server_config.config());
    let connector = TlsConnector::from(client_config.config());

    // 绑定随机端口
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind");
    let addr = listener.local_addr().expect("Failed to get local addr");

    // 启动服务端任务
    let server_task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.expect("Failed to accept");
        let mut tls_stream = acceptor.accept(stream).await.expect("TLS accept failed");

        // 读取客户端消息
        let mut buf = [0u8; 5];
        tls_stream.read_exact(&mut buf).await.expect("Read failed");
        assert_eq!(&buf, b"hello", "Server received unexpected data");

        // 发送响应
        tls_stream.write_all(b"world").await.expect("Write failed");
        tls_stream.flush().await.expect("Flush failed");
    });

    // 客户端连接
    let stream = TcpStream::connect(addr).await.expect("Connect failed");
    let server_name = "localhost"
        .try_into()
        .expect("Invalid server name");
    let mut tls_stream = connector
        .connect(server_name, stream)
        .await
        .expect("TLS connect failed");

    // 发送消息到服务端
    tls_stream.write_all(b"hello").await.expect("Write failed");
    tls_stream.flush().await.expect("Flush failed");

    // 读取服务端响应
    let mut buf = [0u8; 5];
    tls_stream.read_exact(&mut buf).await.expect("Read failed");
    assert_eq!(&buf, b"world", "Client received unexpected data");

    // 等待服务端完成
    server_task.await.expect("Server task failed");
}

/// 测试证书 SAN 不匹配时连接失败
#[tokio::test]
async fn test_tls_handshake_san_mismatch_fails() {
    // 生成只支持 "example.com" 的证书
    let server_cert = TlsCertificate::generate(&["example.com".to_string()])
        .expect("Failed to generate certificate");

    let server_config = TlsServerConfig::new(&server_cert)
        .expect("Failed to create server config");
    let client_config = TlsClientConfig::new(server_cert.cert_der())
        .expect("Failed to create client config");

    let acceptor = TlsAcceptor::from(server_config.config());
    let connector = TlsConnector::from(client_config.config());

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind");
    let addr = listener.local_addr().expect("Failed to get local addr");

    // 服务端接受连接（会成功）
    let server_task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.expect("Failed to accept");
        // 服务端 TLS 握手可能成功也可能失败，取决于客户端何时关闭
        let _ = acceptor.accept(stream).await;
    });

    // 客户端尝试用 "localhost" 连接（应该因 SAN 不匹配而失败）
    let stream = TcpStream::connect(addr).await.expect("Connect failed");
    let server_name = "localhost"
        .try_into()
        .expect("Invalid server name");

    let result = connector.connect(server_name, stream).await;

    // 连接应该失败，因为证书 SAN 不包含 "localhost"
    assert!(result.is_err(), "Connection should fail due to SAN mismatch");

    let _ = server_task.await;
}

/// 测试不受信任的证书连接失败
#[tokio::test]
async fn test_tls_handshake_untrusted_cert_fails() {
    // 生成两个不同的证书
    let server_cert = TlsCertificate::generate(&["localhost".to_string()])
        .expect("Failed to generate server certificate");
    let other_cert = TlsCertificate::generate(&["localhost".to_string()])
        .expect("Failed to generate other certificate");

    let server_config = TlsServerConfig::new(&server_cert)
        .expect("Failed to create server config");
    // 客户端信任另一个证书（不是服务端的）
    let client_config = TlsClientConfig::new(other_cert.cert_der())
        .expect("Failed to create client config");

    let acceptor = TlsAcceptor::from(server_config.config());
    let connector = TlsConnector::from(client_config.config());

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind");
    let addr = listener.local_addr().expect("Failed to get local addr");

    let server_task = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.expect("Failed to accept");
        let _ = acceptor.accept(stream).await;
    });

    let stream = TcpStream::connect(addr).await.expect("Connect failed");
    let server_name = "localhost"
        .try_into()
        .expect("Invalid server name");

    let result = connector.connect(server_name, stream).await;

    // 连接应该失败，因为证书不受信任
    assert!(result.is_err(), "Connection should fail due to untrusted certificate");

    let _ = server_task.await;
}
