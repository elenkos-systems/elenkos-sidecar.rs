/// Model only: this does not implement runtime authentication or authorization.
/// A payroll mutation requires a fresh authenticated session, an explicitly
/// scoped and matching tenant, and administrator authority.
#[derive(Clone, Copy)]
struct PayrollAuthority {
    authenticated: bool,
    tenant_scoped: bool,
    tenant_matches: bool,
    admin: bool,
    session_fresh: bool,
}

fn admitted(authority: PayrollAuthority, payroll_write: bool) -> bool {
    !payroll_write
        || (authority.authenticated
            && authority.tenant_scoped
            && authority.tenant_matches
            && authority.admin
            && authority.session_fresh)
}

fn authority(bits: u8) -> PayrollAuthority {
    PayrollAuthority {
        authenticated: bits & 1 != 0,
        tenant_scoped: bits & 2 != 0,
        tenant_matches: bits & 4 != 0,
        admin: bits & 8 != 0,
        session_fresh: bits & 16 != 0,
    }
}

fn main() {
    let mut checked = 0_u8;
    // Five authorization predicates plus a read/write operation bit.
    for bits in 0_u8..64 {
        let context = authority(bits);
        let payroll_write = bits & 32 != 0;
        let allowed = admitted(context, payroll_write);
        let expected = !payroll_write
            || (context.authenticated
                && context.tenant_scoped
                && context.tenant_matches
                && context.admin
                && context.session_fresh);
        assert_eq!(allowed, expected, "unexpected payroll admission state {bits}");
        checked += 1;
    }
    assert_eq!(checked, 64);
    println!("admin payroll capability model: {checked} states");
}

#[cfg(test)]
mod tests {
    use super::{admitted, authority};

    #[test]
    fn payroll_mutations_require_all_five_capabilities() {
        let mut admitted_cases = 0;
        for bits in 0_u8..32 {
            let allowed = admitted(authority(bits), true);
            let expected = bits == 31;
            assert_eq!(allowed, expected, "unexpected admission state {bits}");
            admitted_cases += u32::from(allowed);
        }
        assert_eq!(admitted_cases, 1);
    }

    #[test]
    fn unprivileged_operations_are_outside_this_payroll_mutation_gate() {
        for bits in 0_u8..32 {
            assert!(admitted(authority(bits), false));
        }
    }
}
