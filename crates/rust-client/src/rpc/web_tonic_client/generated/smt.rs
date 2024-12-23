// This file is @generated by prost-build.
/// An entry in a leaf.
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct SmtLeafEntry {
    #[prost(message, optional, tag = "1")]
    pub key: ::core::option::Option<super::digest::Digest>,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<super::digest::Digest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmtLeafEntries {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<SmtLeafEntry>,
}
/// A leaf in an SMT, sitting at depth 64. A leaf can contain 0, 1 or multiple leaf entries.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmtLeaf {
    #[prost(oneof = "smt_leaf::Leaf", tags = "1, 2, 3")]
    pub leaf: ::core::option::Option<smt_leaf::Leaf>,
}
/// Nested message and enum types in `SmtLeaf`.
pub mod smt_leaf {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Leaf {
        #[prost(uint64, tag = "1")]
        Empty(u64),
        #[prost(message, tag = "2")]
        Single(super::SmtLeafEntry),
        #[prost(message, tag = "3")]
        Multiple(super::SmtLeafEntries),
    }
}
/// The opening of a leaf in an SMT.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SmtOpening {
    #[prost(message, optional, tag = "1")]
    pub path: ::core::option::Option<super::merkle::MerklePath>,
    #[prost(message, optional, tag = "2")]
    pub leaf: ::core::option::Option<SmtLeaf>,
}
