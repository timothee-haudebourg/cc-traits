#[cfg(not(feature="nostd"))]
mod std_collections;

#[cfg(feature="slab")]
mod slab;

#[cfg(feature="smallvec")]
mod smallvec;