// Represents an input — it spends a previous output [`OutPoint`].
#[derive(Debug)]
pub struct TransparentInput {
    pub previous_output: OutPoint,
    pub script_sig: Vec<u8>,
    pub sequence: u32,
}

/// Represents an output, where the value is sent.
#[derive(Debug)]
pub struct TransparentOutput {
    /// Amount in zatoshis.
    pub value: u64,
    /// The locking script.
    pub script_pubkey: Vec<u8>,
}

/// An OutPoint specifies the UTXO being spent.
#[derive(Debug)]
pub struct OutPoint {
    /// The transaction hash.
    pub txid: [u8; 32],
    /// The index (position) of the output within that transaction.
    pub index: u32,
}

#[derive(Debug)]
pub struct TransparentBundle {
    pub inputs: Vec<TransparentInput>,
    pub outputs: Vec<TransparentOutput>,
}
