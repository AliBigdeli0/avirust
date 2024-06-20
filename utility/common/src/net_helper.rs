use futures::Future;
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;

// TcpStreamTrait

pub trait StreamTrait {
    fn ping(&self, addr: SocketAddr) -> impl Future<Output = bool>;
}

struct NetTcpStream {
    open_timeout: u64,
}

impl StreamTrait for NetTcpStream {
    fn ping(&self, addr: SocketAddr) -> impl Future<Output = bool> {
        async move {
            let connection_timeout = Duration::from_secs(self.open_timeout);
            let connect_result = timeout(connection_timeout, TcpStream::connect(addr)).await;

            match connect_result {
                Ok(Ok(_)) => true,
                Ok(Err(_)) => false,
                Err(_) => false, // Timeout
            }
        }
    }
}
// .TcpStreamTrait

pub async fn check_port(addr: SocketAddr, op_timeout: Option<u64>) -> bool {
    let inst_network: &NetTcpStream = &NetTcpStream {
        open_timeout: op_timeout.unwrap_or(1),
    };

    is_tcp_port_open(addr, inst_network).await
}

async fn is_tcp_port_open<T: StreamTrait>(addr: SocketAddr, inst_network: &T) -> bool {
    inst_network.ping(addr).await
}

#[cfg(test)]
mod test_net_helper {
    use super::*;

    #[derive(Debug)]
    struct MockStream;

    impl StreamTrait for MockStream {
        fn ping(&self, _: SocketAddr) -> impl Future<Output = bool> {
            async { false }
        }
    }

    #[tokio::test]
    async fn test_mock_connection() {
        let addr: SocketAddr = "127.0.0.1:60090".parse().expect("error in get page");
        let mock_stream: &MockStream = &MockStream {};

        assert_eq!(
            is_tcp_port_open(addr, mock_stream).await,
            false,
            "the connection does not open"
        )
    }

    #[tokio::test]
    async fn check_failed_connection() {
        let addr: SocketAddr = "172.30.2.241:60090".parse().expect("error in get page");
        let result = check_port(addr, None).await;

        //todo: this is not working
        assert_eq!(result, true, "the connection should not get a valid result")
    }
}
