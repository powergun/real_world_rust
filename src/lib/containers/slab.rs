// explained in
// rust std lib cookbook P/86

// search for rust slab crate
// see
// https://docs.rs/slab/0.4.2/slab/

// a situation in which this is useful is in a connection pool
// if you have multiple clients who wants to access individual
// resources, you can store said resources in a slab and
// provide the clients with their key as a kind of token
