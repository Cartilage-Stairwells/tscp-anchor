use p3_baby_bear::BabyBear;
use p3_field::PackedField;

pub struct Plonky3Adapter;

impl Plonky3Adapter {
    pub fn unpack_packed_slice<'a, P>(packed_slice: &'a [P]) -> &'a [BabyBear]
    where
        P: PackedField<Scalar = BabyBear>,
    {
        P::unpack_slice(packed_slice)
    }
}

#[cfg(feature = "strict-verified")]
pub fn verify_vectorized_layer<P>(
    native_packed: &[P],
    reference_scalars: &[VerifiedBabyBear],
    inputs_a_packed: &[P],
    inputs_b_packed: &[P],
) -> Result<(), VerificationError>
where
    P: PackedField<Scalar = BabyBear>,
{
    let native_scalars = Plonky3Adapter::unpack_packed_slice(native_packed);
    let inputs_a_scalars = Plonky3Adapter::unpack_packed_slice(inputs_a_packed);
    let inputs_b_scalars = Plonky3Adapter::unpack_packed_slice(inputs_b_packed);

    if native_scalars.len() != reference_scalars.len() {
        return Err(VerificationError::new_layout_mismatch(
            native_scalars.len(),
            reference_scalars.len(),
        ));
    }

    for (i, (&n, &r)) in native_scalars.iter().zip(reference_scalars.iter()).enumerate() {
        use p3_field::PrimeField64;
        let native_val = n.to_canonical_u64();
        let reference_val = r.0;
        debug_assert_eq!(
            native_val, reference_val,
            "[VOS DEBUG] Mid-flight lane corruption caught at scalar index {}",
            i
        );
        if native_val != reference_val {
            return Err(VerificationError::new(
                i,
                inputs_a_scalars.get(i).map_or(0, |x| x.to_canonical_u64()),
                inputs_b_scalars.get(i).map_or(0, |x| x.to_canonical_u64()),
                native_val,
                reference_val,
            ));
        }
    }
    Ok(())
}
