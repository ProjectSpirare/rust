use crate::convert::{TryFrom, TryInto};
use crate::fmt;
use crate::io::{self, IoSlice, IoSliceMut};
use crate::mem;
use crate::net::{IpAddr, Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr};
use crate::sys::fd::WasiFd;
use crate::sys::unsupported;
use crate::sys_common::FromInner;
use crate::time::Duration;
use crate::vec::IntoIter;

pub struct TcpStream {
    fd: WasiFd,
}

fn iovec<'a>(a: &'a mut [IoSliceMut<'_>]) -> &'a [wasi::Iovec] {
    assert_eq!(mem::size_of::<IoSliceMut<'_>>(), mem::size_of::<wasi::Iovec>());
    assert_eq!(mem::align_of::<IoSliceMut<'_>>(), mem::align_of::<wasi::Iovec>());
    unsafe { mem::transmute(a) }
}

fn ciovec<'a>(a: &'a [IoSlice<'_>]) -> &'a [wasi::Ciovec] {
    assert_eq!(mem::size_of::<IoSlice<'_>>(), mem::size_of::<wasi::Ciovec>());
    assert_eq!(mem::align_of::<IoSlice<'_>>(), mem::align_of::<wasi::Ciovec>());
    unsafe { mem::transmute(a) }
}

impl TcpStream {
    pub fn connect(addr: io::Result<&SocketAddr>) -> io::Result<TcpStream> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::connect");

        if let SocketAddr::V4(ipv4) = addr.expect("unrapping io::Result<&SocketAddr> failed") {
            let addr: u32 = ipv4.ip().clone().into();
            let port: u16 = ipv4.port();
            let fd = unsafe { wasi::sock_connect(addr, port).unwrap() };
            Ok(Self { fd: unsafe { WasiFd::from_raw(fd) } })
        } else {
            unsupported()
        }
    }

    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::connect_timeout");
        unsupported()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::set_read_timeout");
        unsupported()
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::set_write_timeout");
        unsupported()
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::read_timeout");
        unsupported()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::write_timeout");
        unsupported()
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::peek");
        unsupported()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.read_vectored(&mut [IoSliceMut::new(buf)])
    }

    pub fn read_vectored(&self, iov: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        Ok(unsafe { wasi::sock_recv(self.fd.as_raw(), iovec(iov), 0).unwrap().0 })
    }

    pub fn is_read_vectored(&self) -> bool {
        true
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.write_vectored(&[IoSlice::new(buf)])
    }

    pub fn write_vectored(&self, iov: &[IoSlice<'_>]) -> io::Result<usize> {
        Ok(unsafe { wasi::sock_send(self.fd.as_raw(), ciovec(iov), 0).unwrap() })
    }

    pub fn is_write_vectored(&self) -> bool {
        true
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::peer_addr");
        unsupported()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::socket_addr");
        unsupported()
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::shutdown");
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::duplicate");
        unsupported()
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::set_nodelay");
        unsupported()
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::nodelay");
        unsupported()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::set_ttl");
        unsupported()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::ttl");
        unsupported()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::take_error");
        unsupported()
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        println!("rust_stdlib_sys_wasi_net::TcpStream::set_nonblocking");
        unsupported()
    }

    pub fn fd(&self) -> &WasiFd {
        &self.fd
    }

    pub fn into_fd(self) -> WasiFd {
        self.fd
    }
}

impl FromInner<u32> for TcpStream {
    fn from_inner(fd: u32) -> TcpStream {
        unsafe { TcpStream { fd: WasiFd::from_raw(fd) } }
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpStream").field("fd", &self.fd.as_raw()).finish()
    }
}

pub struct TcpListener {
    fd: WasiFd,
}

impl TcpListener {
    pub fn bind(_: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
        println!("rust_stdlib_sys_wasi_net::TcpListener::bind");
        unsupported()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        unsupported()
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        unsupported()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unsupported()
    }

    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unsupported()
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn fd(&self) -> &WasiFd {
        &self.fd
    }

    pub fn into_fd(self) -> WasiFd {
        self.fd
    }
}

impl FromInner<u32> for TcpListener {
    fn from_inner(fd: u32) -> TcpListener {
        unsafe { TcpListener { fd: WasiFd::from_raw(fd) } }
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TcpListener").field("fd", &self.fd.as_raw()).finish()
    }
}

pub struct UdpSocket {
    fd: WasiFd,
}

impl UdpSocket {
    pub fn bind(_: io::Result<&SocketAddr>) -> io::Result<UdpSocket> {
        unsupported()
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        unsupported()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        unsupported()
    }

    pub fn recv_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unsupported()
    }

    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unsupported()
    }

    pub fn send_to(&self, _: &[u8], _: &SocketAddr) -> io::Result<usize> {
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        unsupported()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        unsupported()
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        unsupported()
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        unsupported()
    }

    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        unsupported()
    }

    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        unsupported()
    }

    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unsupported()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unsupported()
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn recv(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn send(&self, _: &[u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn connect(&self, _: io::Result<&SocketAddr>) -> io::Result<()> {
        unsupported()
    }

    pub fn fd(&self) -> &WasiFd {
        &self.fd
    }

    pub fn into_fd(self) -> WasiFd {
        self.fd
    }
}

impl FromInner<u32> for UdpSocket {
    fn from_inner(fd: u32) -> UdpSocket {
        unsafe { UdpSocket { fd: WasiFd::from_raw(fd) } }
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UdpSocket").field("fd", &self.fd.as_raw()).finish()
    }
}

pub struct LookupHost(IntoIter<SocketAddr>, u16);

impl LookupHost {
    pub fn port(&self) -> u16 {
        self.1
    }
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl TryFrom<&str> for LookupHost {
    type Error = io::Error;

    fn try_from(s: &str) -> io::Result<LookupHost> {
        println!("rust_stdlib_sys_wasi_net::LookupHost::try_from &str {:#?}", s);

        macro_rules! try_opt {
            ($e:expr, $msg:expr) => {
                match $e {
                    Some(r) => r,
                    None => return Err(io::Error::new(io::ErrorKind::InvalidInput, $msg)),
                }
            };
        }

        // split the string by ':' and convert the second part to u16
        let mut parts_iter = s.rsplitn(2, ':');
        let port_str = try_opt!(parts_iter.next(), "invalid socket address");
        let host = try_opt!(parts_iter.next(), "invalid socket address");
        let port: u16 = try_opt!(port_str.parse().ok(), "invalid port value");

        (host, port).try_into()
    }
}

impl<'a> TryFrom<(&'a str, u16)> for LookupHost {
    type Error = io::Error;

    fn try_from((host, port): (&'a str, u16)) -> io::Result<LookupHost> {
        println!("rust_stdlib_sys_wasi_net::LookupHost::try_from tuple: {:#?}:{:#?}", host, port);

        let mut addrs = vec![];

        // let addr = SocketAddrV4::new(Ipv4Addr::from(host), port);
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
        addrs.push(socket);
        Ok(LookupHost(addrs.into_iter(), port))

        // unsupported()
    }
}

#[allow(nonstandard_style)]
pub mod netc {
    pub const AF_INET: u8 = 0;
    pub const AF_INET6: u8 = 1;
    pub type sa_family_t = u8;

    #[derive(Copy, Clone)]
    pub struct in_addr {
        pub s_addr: u32,
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr_in {
        pub sin_family: sa_family_t,
        pub sin_port: u16,
        pub sin_addr: in_addr,
    }

    #[derive(Copy, Clone)]
    pub struct in6_addr {
        pub s6_addr: [u8; 16],
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: u16,
        pub sin6_addr: in6_addr,
        pub sin6_flowinfo: u32,
        pub sin6_scope_id: u32,
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr {}

    pub type socklen_t = usize;
}
