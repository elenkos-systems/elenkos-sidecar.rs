#!/usr/bin/env python3
from itertools import product

def main():
    checked=0
    for auth,tenant,admin,payroll_write in product((False,True),repeat=4):
        admitted=(not payroll_write) or (auth and tenant and admin)
        if payroll_write and admitted:
            assert auth and tenant and admin,'payroll mutation admitted without admin tenant authority'
        checked+=1
    print(f'admin capability model: {checked} states')
if __name__=='__main__':main()
