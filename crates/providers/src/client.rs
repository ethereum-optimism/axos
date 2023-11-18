// #![feature(async_fn_in_trait)]
// use embedded_io_adapters::tokio_1::FromTokio;
// use embedded_io_async::BufRead;
// use embedded_nal_async::{AddrType, IpAddr, Ipv4Addr};
//
// /// Loopback DNS resolver
// #[derive(Debug)]
// pub struct LoopbackDns;
//
//
// impl embedded_nal_async::Dns for LoopbackDns {
//     type Error = TestError;
//
//     async fn get_host_by_name(&self, _: &str, _: AddrType) -> Result<IpAddr, Self::Error> {
//         Ok(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
//     }
//
//     async fn get_host_by_address(&self, _: IpAddr, _: &mut [u8]) -> Result<usize, Self::Error> {
//         Err(TestError)
//     }
// }
//
// /// Tokio TCP
// #[derive(Debug)]
// pub struct TokioTcp;
//
// /// An embedded io Error
// #[derive(Debug)]
// pub enum TcpError {
//     /// Some other error
//     Other,
// }
//
// impl embedded_io::Error for TcpError {
//     fn kind(&self) -> embedded_io::ErrorKind {
//         embedded_io::ErrorKind::Other
//     }
// }
//
// impl embedded_nal_async::TcpConnect for TokioTcp {
//     type Error = std::io::Error;
//     type Connection<'m> = FromTokio<TcpStream>;
//
//     async fn connect<'m>(
//         &'m self,
//         remote: embedded_nal_async::SocketAddr,
//     ) -> Result<Self::Connection<'m>, Self::Error> {
//         let ip = match remote {
//             embedded_nal_async::SocketAddr::V4(a) => a.ip().octets().into(),
//             embedded_nal_async::SocketAddr::V6(a) => a.ip().octets().into(),
//         };
//         let remote = SocketAddr::new(ip, remote.port());
//         let stream = TcpStream::connect(remote).await?;
//         let stream = FromTokio::new(stream);
//         Ok(stream)
//     }
// }
