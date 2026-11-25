#!/usr/bin/env python3
"""Pure payroll-capability model; not a runtime authorization implementation."""
from itertools import product


def admitted(authenticated, tenant_scoped, tenant_matches, admin, session_fresh, payroll_write):
    return (not payroll_write or (
        authenticated and tenant_scoped and tenant_matches and admin and session_fresh
    ))


def main():
    checked = 0
    admitted_writes = 0
    for auth, scoped, matches, admin, fresh, write in product((False, True), repeat=6):
        allowed = admitted(auth, scoped, matches, admin, fresh, write)
        expected = not write or all((auth, scoped, matches, admin, fresh))
        assert allowed == expected, "payroll mutation diverged from full authority predicate"
        if write and allowed:
            assert all((auth, scoped, matches, admin, fresh)), (
                "payroll mutation admitted without matching tenant and fresh admin session"
            )
            admitted_writes += 1
        checked += 1
    assert checked == 64
    assert admitted_writes == 1
    print(f"admin payroll capability model: {checked} states; {admitted_writes} admitted write")


if __name__ == "__main__":
    main()
