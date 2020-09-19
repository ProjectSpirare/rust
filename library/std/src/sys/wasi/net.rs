#![deny(unsafe_op_in_unsafe_fn)]

use crate::convert::TryFrom;
use crate::fmt;
use crate::io::{self, IoSlice, IoSliceMut};
// use crate::mem;
use crate::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr};
use crate::sys::fd::WasiFd;
use crate::sys::{unsupported, Void};
use crate::sys_common::FromInner;
use crate::time::Duration;

// fn iovec<'a>(a: &'a mut [IoSliceMut<'_>]) -> &'a [wasi::Iovec] {
//     assert_eq!(mem::size_of::<IoSliceMut<'_>>(), mem::size_of::<wasi::Iovec>());
//     assert_eq!(mem::align_of::<IoSliceMut<'_>>(), mem::align_of::<wasi::Iovec>());
//     // SAFETY: `IoSliceMut` and `IoVec` have exactly the same memory layout
//     unsafe { mem::transmute(a) }
// }

// fn ciovec<'a>(a: &'a [IoSlice<'_>]) -> &'a [wasi::Ciovec] {
//     assert_eq!(mem::size_of::<IoSlice<'_>>(), mem::size_of::<wasi::Ciovec>());
//     assert_eq!(mem::align_of::<IoSlice<'_>>(), mem::align_of::<wasi::Ciovec>());
//     // SAFETY: `IoSlice` and `CIoVec` have exactly the same memory layout
//     unsafe { mem::transmute(a) }
// }

pub struct TcpStream {
    fd: WasiFd,
}

// TODO
// move to fd.rs
pub const STDPOOL_FD: wasi::Fd = 0x3;

impl TcpStream {
    pub fn connect(addr: io::Result<&SocketAddr>) -> io::Result<TcpStream> {
        if let SocketAddr::V4(ipv4) = addr? {
            // let mut addr = ipv4.ip().clone().into() as u8;
            let port: u16 = ipv4.port();

            // TODO
            // actually parse the IP address above
            let mut addr = wasi::Addr {
                tag: wasi::ADDR_TYPE_IP4,
                u: wasi::AddrU {
                    ip4: wasi::AddrIp4Port {
                        addr: wasi::AddrIp4 { n0: 127, n1: 0, h0: 0, h1: 1 },
                        port,
                    },
                },
            };

            // TODO
            // the
            unsafe {
                let sd = wasi::sock_open(
                    STDPOOL_FD,
                    wasi::ADDRESS_FAMILY_INET4,
                    wasi::SOCK_TYPE_SOCKET_STREAM,
                )
                .unwrap();

                wasi::sock_connect(sd, &mut addr as *mut wasi::Addr).unwrap();
                Ok(Self { fd: WasiFd::from_raw(sd) })
            }
        } else {
            unsupported()
        }
    }

    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
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

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.read_vectored(&mut [IoSliceMut::new(buf)])
    }

    pub fn read_vectored(&self, _: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        // Ok(self.fd.sock_recv(iov.as_ptr(), 0).unwrap())
        //Ok(unsafe { wasi::sock_recv(self.fd.as_raw(), iovec(iov), 0).unwrap() })
        unsupported()
    }

    pub fn is_read_vectored(&self) -> bool {
        true
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.write_vectored(&[IoSlice::new(buf)])
    }

    pub fn write_vectored(&self, _: &[IoSlice<'_>]) -> io::Result<usize> {
        // Ok(unsafe { wasi::sock_send(self.fd.as_raw(), ciovec(iov), 0).unwrap() })
        unsupported()
    }

    pub fn is_write_vectored(&self) -> bool {
        true
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        unsupported()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        unsupported()
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        unsupported()
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn nodelay(&self) -> io::Result<bool> {
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

pub struct LookupHost(Void);

impl LookupHost {
    pub fn port(&self) -> u16 {
        match self.0 {}
    }
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        match self.0 {}
    }
}

impl<'a> TryFrom<&'a str> for LookupHost {
    type Error = io::Error;

    fn try_from(_v: &'a str) -> io::Result<LookupHost> {
        unsupported()
    }
}

impl<'a> TryFrom<(&'a str, u16)> for LookupHost {
    type Error = io::Error;

    fn try_from(_v: (&'a str, u16)) -> io::Result<LookupHost> {
        unsupported()
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
