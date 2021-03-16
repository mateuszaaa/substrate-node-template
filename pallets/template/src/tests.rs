use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn solve_non_existing_problem() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            PrimeProblem::solve_problem(Origin::signed(1), 6, 6, 0),
            Error::<Test>::ProblemNotExists
        );
    });
}

#[test]
fn fail_to_register_problem_with_not_enought_funds() {
    new_test_ext().execute_with(|| {
        assert_noop!(
            PrimeProblem::register_problem(Origin::signed(1), 6, 200000),
            Error::<Test>::NotEnoughtFunds
        );
    });
}

#[test]
fn test_solve_problem_using_prime_number() {
    new_test_ext().execute_with(|| {
        assert_eq!(Balances::free_balance(&1), 100000);
        assert_eq!(Balances::free_balance(PrimeProblem::account_id()), 100000);
        assert_ok!(PrimeProblem::register_problem(Origin::signed(1), 6, 50000));
        assert_eq!(Balances::free_balance(&1), 50000);

        assert_eq!(Balances::free_balance(&2), 100000);
        assert_ok!(PrimeProblem::solve_problem(Origin::signed(2), 6, 2, 3));
        // 50'000 spent on reward
        assert_eq!(Balances::free_balance(&1), 50000);
        // 50'000 * 0.8 = 40'000 - solution reward
        assert_eq!(Balances::free_balance(&2), 140000);
        // 50'000 * 0.2 = 10'000 - fee
        assert_eq!(Balances::free_balance(PrimeProblem::account_id()), 110000);
    });
}

#[test]
fn test_solve_problem_using_non_prime_number() {
    new_test_ext().execute_with(|| {
        assert_ok!(PrimeProblem::register_problem(Origin::signed(1), 8, 50000));
        assert_noop!(
            PrimeProblem::solve_problem(Origin::signed(2), 8, 2, 4),
            Error::<Test>::NotPrimeNumber
        );
    });
}

#[test]
fn fail_to_solve_problem_using_wrong_solution() {
    new_test_ext().execute_with(|| {
        assert_ok!(PrimeProblem::register_problem(Origin::signed(1), 6, 50000));
        assert_noop!(
            PrimeProblem::solve_problem(Origin::signed(2), 6, 3, 5),
            Error::<Test>::WrongSolution
        );
    });
}

#[test]
fn fail_to_register_the_same_problem_twice() {
    new_test_ext().execute_with(|| {
        assert_ok!(PrimeProblem::register_problem(Origin::signed(1), 6, 10000));
        assert_noop!(
            PrimeProblem::register_problem(Origin::signed(1), 6, 10000),
            Error::<Test>::ProblemAlreadyExists
        );
    });
}

#[test]
fn fail_to_solve_the_same_problem_twice() {
    new_test_ext().execute_with(|| {
        assert_ok!(PrimeProblem::register_problem(Origin::signed(1), 6, 50000));
        assert_ok!(PrimeProblem::solve_problem(Origin::signed(2), 6, 2, 3));

        assert_noop!(
            PrimeProblem::solve_problem(Origin::signed(2), 6, 2, 3),
            Error::<Test>::ProblemAlreadySolved
        );
    });
}
