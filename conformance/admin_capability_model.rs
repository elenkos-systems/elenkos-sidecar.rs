fn admitted(authenticated: bool, tenant_scoped: bool, admin: bool, payroll_write: bool) -> bool {
    !payroll_write || (authenticated && tenant_scoped && admin)
}

fn main() {
    let mut checked = 0_u8;
    for bits in 0_u8..16 {
        let authenticated = bits & 1 != 0;
        let tenant_scoped = bits & 2 != 0;
        let admin = bits & 4 != 0;
        let payroll_write = bits & 8 != 0;

        let is_admitted = admitted(authenticated, tenant_scoped, admin, payroll_write);
        if payroll_write && is_admitted {
            assert!(authenticated && tenant_scoped && admin,
                "payroll mutation admitted without authenticated tenant admin authority");
        }
        checked += 1;
    }

    assert_eq!(checked, 16);
    println!("admin capability model: {checked} states");
}

#[cfg(test)]
mod tests {
    use super::admitted;

    #[test]
    fn payroll_write_fails_closed_without_full_authority() {
        for authenticated in [false, true] {
            for tenant_scoped in [false, true] {
                for admin in [false, true] {
                    let expected = authenticated && tenant_scoped && admin;
                    assert_eq!(admitted(authenticated, tenant_scoped, admin, true), expected);
                }
            }
        }
    }

    #[test]
    fn non_payroll_operations_are_outside_this_capability_gate() {
        for authenticated in [false, true] {
            for tenant_scoped in [false, true] {
                for admin in [false, true] {
                    assert!(admitted(authenticated, tenant_scoped, admin, false));
                }
            }
        }
    }
}
