/*!
 * TSCP Polynomial Commitment Crate (PCC)
 *
 * Minimal compiling version.
 */

// Re-exports
pub use p3_commit::Pcs;
pub use p3_fri::TwoAdicFriPcs;
pub use p3_fri::{FriParameters, FriProof};
pub use p3_field::PrimeField;

/// Convenient alias used throughout TSCP
pub use Pcs as PolynomialCommitment;

/// Main constructor (the one you wanted)
/// Returns TwoAdicFriPcs directly — the simplest and most compatible form.
pub fn new_tscp_pcs<Val, Dft, InputMmcs, FriMmcs>(
    dft: Dft,
    input_mmcs: InputMmcs,
    _fri_mmcs: FriMmcs,
    fri_params: FriParameters<FriMmcs>,
) -> TwoAdicFriPcs<Val, Dft, InputMmcs, FriMmcs>
where
    Val: p3_field::TwoAdicField,
    Dft: p3_dft::TwoAdicSubgroupDft<Val>,
    InputMmcs: p3_commit::Mmcs<Val>,
    FriMmcs: p3_commit::Mmcs<Val>,
{
    TwoAdicFriPcs::new(dft, input_mmcs, fri_params)
}
