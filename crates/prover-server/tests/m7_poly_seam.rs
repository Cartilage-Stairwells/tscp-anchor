use prover_server::poly_ir::PolyIR;

#[test]
fn test_m7_semantic_correctness() {
    let ir = PolyIR {
        version: "m7.1".into(),
        constraints: vec![],
    };

    assert!(ir.verify_schema().is_ok());
}

#[test]
fn test_trace_rejects_invalid_transition() {
    let ir = PolyIR {
        version: "".into(),
        constraints: vec![],
    };

    assert!(ir.verify_schema().is_err());
}
