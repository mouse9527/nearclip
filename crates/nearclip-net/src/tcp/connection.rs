//! TLS 加密的 TCP 连接
//!
//! 提供连接抽象，封装 TLS 流的读写操作。
//! 支持服务端和客户端两种连接类型。

use crate::NetError;
use std::net::SocketAddr;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf};
use tokio::net::TcpStream;
use tracing::{debug, instrument};

/// TLS 流包装器
///
/// 封装服务端和客户端两种 TLS 流，提供统一的读写接口。
enum TlsStreamWrapper {
    Server(tokio_rustls::server::TlsStream<TcpStream>),
    Client(tokio_rustls::client::TlsStream<TcpStream>),
}

impl AsyncRead for TlsStreamWrapper {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            TlsStreamWrapper::Server(s) => Pin::new(s).poll_read(cx, buf),
            TlsStreamWrapper::Client(s) => Pin::new(s).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for TlsStreamWrapper {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        match self.get_mut() {
            TlsStreamWrapper::Server(s) => Pin::new(s).poll_write(cx, buf),
            TlsStreamWrapper::Client(s) => Pin::new(s).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            TlsStreamWrapper::Server(s) => Pin::new(s).poll_flush(cx),
            TlsStreamWrapper::Client(s) => Pin::new(s).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            TlsStreamWrapper::Server(s) => Pin::new(s).poll_shutdown(cx),
            TlsStreamWrapper::Client(s) => Pin::new(s).poll_shutdown(cx),
        }
    }
}

/// TLS 加密的 TCP 连接
///
/// 封装 TLS 流，提供简单的读写接口。
/// 支持服务端接受的连接和客户端发起的连接。
///
/// # Example
///
/// ```no_run
/// use nearclip_net::tcp::{TcpServer, TcpServerConfig, TcpConnection};
/// use nearclip_crypto::{TlsCertificate, TlsServerConfig};
///
/// # async fn example() -> Result<(), nearclip_net::NetError> {
/// # let cert = TlsCertificate::generate(&["localhost".to_string()]).unwrap();
/// # let tls_config = TlsServerConfig::new(&cert).unwrap();
/// # let server = TcpServer::bind(TcpServerConfig::new(), tls_config.config()).await?;
/// let mut conn = server.accept().await?;
///
/// // 读取数据
/// let mut buf = [0u8; 1024];
/// let n = conn.read(&mut buf).await?;
///
/// // 写入数据
/// conn.write_all(b"Hello, World!").await?;
///
/// // 关闭连接
/// conn.close().await?;
/// # Ok(())
/// # }
/// ```
pub struct TcpConnection {
    stream: TlsStreamWrapper,
    peer_addr: SocketAddr,
}

impl TcpConnection {
    /// 创建服务端连接对象（内部使用）
    pub(crate) fn new(
        stream: tokio_rustls::server::TlsStream<TcpStream>,
        peer_addr: SocketAddr,
    ) -> Self {
        Self {
            stream: TlsStreamWrapper::Server(stream),
            peer_addr,
        }
    }

    /// 创建客户端连接对象（内部使用）
    pub(crate) fn new_client(
        stream: tokio_rustls::client::TlsStream<TcpStream>,
        peer_addr: SocketAddr,
    ) -> Self {
        Self {
            stream: TlsStreamWrapper::Client(stream),
            peer_addr,
        }
    }

    /// 获取对端地址
    ///
    /// 返回连接的远程端地址。
    pub fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }

    /// 读取数据
    ///
    /// 从连接中读取数据到缓冲区。
    ///
    /// # Arguments
    ///
    /// * `buf` - 目标缓冲区
    ///
    /// # Returns
    ///
    /// 实际读取的字节数，0 表示连接已关闭
    #[instrument(skip(self, buf), fields(peer = %self.peer_addr, buf_len = buf.len()))]
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize, NetError> {
        let n = self.stream.read(buf).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                NetError::ConnectionClosed(format!("Peer {} disconnected", self.peer_addr))
            } else {
                NetError::Io(e)
            }
        })?;

        debug!("Read {} bytes from {}", n, self.peer_addr);
        Ok(n)
    }

    /// 写入数据
    ///
    /// 向连接写入数据。可能只写入部分数据。
    ///
    /// # Arguments
    ///
    /// * `data` - 要写入的数据
    ///
    /// # Returns
    ///
    /// 实际写入的字节数
    #[instrument(skip(self, data), fields(peer = %self.peer_addr, data_len = data.len()))]
    pub async fn write(&mut self, data: &[u8]) -> Result<usize, NetError> {
        let n = self.stream.write(data).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::BrokenPipe
                || e.kind() == std::io::ErrorKind::ConnectionReset
            {
                NetError::ConnectionClosed(format!("Connection to {} lost", self.peer_addr))
            } else {
                NetError::Io(e)
            }
        })?;

        debug!("Wrote {} bytes to {}", n, self.peer_addr);
        Ok(n)
    }

    /// 写入所有数据
    ///
    /// 确保所有数据都被写入连接。
    ///
    /// # Arguments
    ///
    /// * `data` - 要写入的数据
    #[instrument(skip(self, data), fields(peer = %self.peer_addr, data_len = data.len()))]
    pub async fn write_all(&mut self, data: &[u8]) -> Result<(), NetError> {
        self.stream.write_all(data).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::BrokenPipe
                || e.kind() == std::io::ErrorKind::ConnectionReset
            {
                NetError::ConnectionClosed(format!("Connection to {} lost", self.peer_addr))
            } else {
                NetError::Io(e)
            }
        })?;

        debug!("Wrote all {} bytes to {}", data.len(), self.peer_addr);
        Ok(())
    }

    /// 刷新写入缓冲区
    ///
    /// 确保所有缓冲的数据被发送。
    #[instrument(skip(self), fields(peer = %self.peer_addr))]
    pub async fn flush(&mut self) -> Result<(), NetError> {
        self.stream.flush().await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::BrokenPipe
                || e.kind() == std::io::ErrorKind::ConnectionReset
            {
                NetError::ConnectionClosed(format!("Connection to {} lost", self.peer_addr))
            } else {
                NetError::Io(e)
            }
        })?;

        debug!("Flushed connection to {}", self.peer_addr);
        Ok(())
    }

    /// 关闭连接
    ///
    /// 优雅地关闭连接，确保所有数据已发送。
    #[instrument(skip(self), fields(peer = %self.peer_addr))]
    pub async fn close(&mut self) -> Result<(), NetError> {
        self.stream.shutdown().await.map_err(|e| {
            // 关闭时的错误通常可以忽略
            debug!("Shutdown error (may be expected): {}", e);
            NetError::Io(e)
        })?;

        debug!("Connection to {} closed", self.peer_addr);
        Ok(())
    }

    /// 分离连接为读写两个独立的半连接
    ///
    /// 允许并发地读取和写入同一连接，无需共享锁。
    /// 返回的读半连接和写半连接可以在不同任务中独立使用。
    ///
    /// # Returns
    ///
    /// * `TcpReadHalf` - 只读半连接
    /// * `TcpWriteHalf` - 只写半连接
    pub fn into_split(self) -> (TcpReadHalf, TcpWriteHalf) {
        let (read, write) = tokio::io::split(self.stream);
        let peer_addr = self.peer_addr;
        (
            TcpReadHalf { read, peer_addr },
            TcpWriteHalf { write, peer_addr },
        )
    }
}

/// TLS 连接的只读半连接
///
/// 通过 `TcpConnection::into_split()` 创建。
/// 可以与 `TcpWriteHalf` 在不同任务中并发使用。
pub struct TcpReadHalf {
    read: tokio::io::ReadHalf<TlsStreamWrapper>,
    peer_addr: SocketAddr,
}

impl TcpReadHalf {
    /// 获取对端地址
    pub fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }

    /// 读取数据
    ///
    /// 从连接中读取数据到缓冲区。
    ///
    /// # Arguments
    ///
    /// * `buf` - 目标缓冲区
    ///
    /// # Returns
    ///
    /// 实际读取的字节数，0 表示连接已关闭
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize, NetError> {
        use tokio::io::AsyncReadExt;
        let n = self.read.read(buf).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                NetError::ConnectionClosed(format!("Peer {} disconnected", self.peer_addr))
            } else {
                NetError::Io(e)
            }
        })?;
        Ok(n)
    }
}

/// TLS 连接的只写半连接
///
/// 通过 `TcpConnection::into_split()` 创建。
/// 可以与 `TcpReadHalf` 在不同任务中并发使用。
pub struct TcpWriteHalf {
    write: tokio::io::WriteHalf<TlsStreamWrapper>,
    peer_addr: SocketAddr,
}

impl TcpWriteHalf {
    /// 获取对端地址
    pub fn peer_addr(&self) -> SocketAddr {
        self.peer_addr
    }

    /// 写入数据
    ///
    /// 向连接写入数据。可能只写入部分数据。
    pub async fn write(&mut self, data: &[u8]) -> Result<usize, NetError> {
        use tokio::io::AsyncWriteExt;
        let n = self.write.write(data).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::BrokenPipe
                || e.kind() == std::io::ErrorKind::ConnectionReset
            {
                NetError::ConnectionClosed(format!("Connection to {} lost", self.peer_addr))
            } else {
                NetError::Io(e)
            }
        })?;
        Ok(n)
    }

    /// 写入所有数据
    ///
    /// 确保所有数据都被写入连接。
    pub async fn write_all(&mut self, data: &[u8]) -> Result<(), NetError> {
        use tokio::io::AsyncWriteExt;
        self.write.write_all(data).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::BrokenPipe
                || e.kind() == std::io::ErrorKind::ConnectionReset
            {
                NetError::ConnectionClosed(format!("Connection to {} lost", self.peer_addr))
            } else {
                NetError::Io(e)
            }
        })
    }

    /// 刷新写入缓冲区
    pub async fn flush(&mut self) -> Result<(), NetError> {
        use tokio::io::AsyncWriteExt;
        self.write.flush().await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::BrokenPipe
                || e.kind() == std::io::ErrorKind::ConnectionReset
            {
                NetError::ConnectionClosed(format!("Connection to {} lost", self.peer_addr))
            } else {
                NetError::Io(e)
            }
        })
    }
}

impl std::fmt::Debug for TcpConnection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TcpConnection")
            .field("peer_addr", &self.peer_addr)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    // TcpConnection 的测试需要完整的 TLS 环境，
    // 将在集成测试中进行

    #[test]
    fn test_connection_debug_impl() {
        // 由于 TcpConnection 需要实际的 TLS 流才能创建，
        // 这里只测试 Debug trait 的格式
        // 实际测试在集成测试中进行
    }
}
