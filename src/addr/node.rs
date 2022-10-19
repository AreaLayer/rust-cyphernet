use crate::addr::SocketAddr;
use std::net;
use std::net::ToSocketAddrs;
use std::str::FromStr;
use std::{fmt, io, option};

use super::{Addr, AddrParseError, UniversalAddr};
use crate::crypto::{Ec, EcPrivKey, EcPubKey};

#[derive(Debug, Display, Error, From)]
#[display(doc_comments)]
pub enum PeerAddrParseError<E: Ec + fmt::Debug + ?Sized>
where
    E::PubKey: FromStr,
    <E::PubKey as FromStr>::Err: std::error::Error,
{
    #[from]
    #[from(net::AddrParseError)]
    #[display(inner)]
    Addr(AddrParseError),

    /// invalid peer key. Details: {0}
    Key(<E::PubKey as FromStr>::Err),

    /// invalid peer address format. Peer address must contain peer key and peer
    /// network address, separated by '@'
    InvalidFormat,
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[display(inner)]
pub struct NodeId<E: Ec + ?Sized>(E::PubKey);

impl<E: Ec + ?Sized> NodeId<E> {
    pub fn from_public_key(pk: E::PubKey) -> Self {
        Self(pk)
    }

    pub fn from_raw(raw: <E::PubKey as EcPubKey<E>>::Raw) -> Self {
        Self(E::PubKey::from_raw(raw))
    }

    pub fn into_raw(self) -> <E::PubKey as EcPubKey<E>>::Raw {
        self.0.into_raw()
    }
}

impl<E: Ec + ?Sized> FromStr for NodeId<E>
where
    E::PubKey: FromStr,
{
    type Err = <E::PubKey as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        E::PubKey::from_str(s).map(Self)
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Display)]
#[display("{pubkey}@{addr}")]
pub struct PeerAddr<E: Ec + ?Sized, A: Addr = UniversalAddr> {
    pubkey: NodeId<E>,
    addr: A,
}

impl<E: Ec + fmt::Debug + ?Sized, A: Addr> Addr for PeerAddr<E, A>
where
    E::PubKey: FromStr,
    <E::PubKey as FromStr>::Err: std::error::Error,
    <A as FromStr>::Err: Into<PeerAddrParseError<E>>,
{
    fn port(&self) -> u16 {
        self.addr.port()
    }
}

impl<E: Ec + fmt::Debug + ?Sized, A: Addr> FromStr for PeerAddr<E, A>
where
    E::PubKey: FromStr,
    <E::PubKey as FromStr>::Err: std::error::Error,
    <A as FromStr>::Err: Into<PeerAddrParseError<E>>,
{
    type Err = PeerAddrParseError<E>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((pk, addr)) = s.split_once('@') {
            Ok(PeerAddr {
                pubkey: NodeId::from_str(pk).map_err(PeerAddrParseError::Key)?,
                addr: A::from_str(addr).map_err(<A as FromStr>::Err::into)?,
            })
        } else {
            Err(PeerAddrParseError::InvalidFormat)
        }
    }
}

impl<E: Ec + ?Sized, const DEFAULT_PORT: u16> From<PeerAddr<E, SocketAddr<DEFAULT_PORT>>>
    for PeerAddr<E, net::SocketAddr>
{
    fn from(peer: PeerAddr<E, SocketAddr<DEFAULT_PORT>>) -> Self {
        PeerAddr {
            addr: peer.addr.into(),
            pubkey: peer.pubkey,
        }
    }
}

impl<E: Ec + ?Sized, A: Addr + Into<net::SocketAddr>> From<PeerAddr<E, A>> for net::SocketAddr {
    fn from(peer: PeerAddr<E, A>) -> Self {
        peer.addr.into()
    }
}

impl<'a, E: Ec + ?Sized, A> PeerAddr<E, A>
where
    A: Addr + 'a,
    &'a A: Into<net::SocketAddr>,
{
    pub fn to_socket_addr(&'a self) -> net::SocketAddr {
        (&self.addr).into()
    }
}

impl<E: Ec + ?Sized, A> ToSocketAddrs for PeerAddr<E, A>
where
    A: Addr + ToSocketAddrs,
{
    type Iter = A::Iter;

    fn to_socket_addrs(&self) -> io::Result<A::Iter> {
        self.addr.to_socket_addrs()
    }
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
pub struct LocalNode<E: Ec + ?Sized> {
    privkey: E::PrivKey,
    pubkey: E::PubKey,
}

impl<E: Ec + ?Sized> LocalNode<E> {
    pub fn from(sk: E::PrivKey) -> Self {
        let pk = sk.to_public_key();
        LocalNode {
            privkey: sk,
            pubkey: pk,
        }
    }

    pub fn id(self) -> NodeId<E> {
        NodeId::from_public_key(self.pubkey)
    }

    pub fn private_key(self) -> E::PrivKey {
        self.privkey
    }
}
