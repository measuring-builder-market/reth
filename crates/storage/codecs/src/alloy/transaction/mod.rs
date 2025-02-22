mod eip1559;
mod eip2930;
mod legacy;

#[cfg(test)]
mod tests {

    // each value in the database has an extra field named flags that encodes metadata about other
    // fields in the value, e.g. offset and length.
    //
    // this check is to ensure we do not inadvertently add too many fields to a struct which would
    // expand the flags field and break backwards compatibility

    use super::{eip1559::TxEip1559, eip2930::TxEip2930, legacy::TxLegacy};

    #[test]
    fn test_ensure_backwards_compatibility() {
        assert_eq!(TxLegacy::bitflag_encoded_bytes(), 3);
        assert_eq!(TxEip1559::bitflag_encoded_bytes(), 4);
        assert_eq!(TxEip2930::bitflag_encoded_bytes(), 3);
    }
}
