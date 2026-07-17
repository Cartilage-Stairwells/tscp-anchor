pub mod edia;

#[cfg(test)]
mod tests {
    use crate::edia::*;

    #[test]
    fn sanity_wire_test() {
        let mut agent = EdiaAgent::new(10);
        assert!(agent.ingest_telemetry().is_ok());
    }
}
pub mod poly_ir;
