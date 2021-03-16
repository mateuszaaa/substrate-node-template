#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::Currency;
use frame_support::traits::ReservableCurrency;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, StorageMap};
use frame_system::ensure_signed;
use sp_runtime::{traits::AccountIdConversion, ModuleId};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    type Currency: ReservableCurrency<Self::AccountId>;
}

decl_storage! {
    trait Store for Module<T: Config> as TemplateModule {
        /// The storage item for our proofs.
        /// It maps a proof to the user who made the claim and when they made it.
        Proofs: map hasher(identity) Vec<u8> => (T::AccountId, T::BlockNumber);
        /// maps problem to reward
        Problems: map hasher(identity) u32  => (T::AccountId, BalanceOf<T>);
        /// maps problem to solution
        SolvedProblems: map hasher(identity) u32  => (u32, u32);
    }
}

decl_event! {
    pub enum Event<T> where
        AccountId = <T as frame_system::Config>::AccountId,
        {
        /// Event emitted when new problem is created
        ProblemCreated(AccountId, u32),
        /// Event emitted when one of the problems has been solved
        ProblemSolved(AccountId, u32),
    }
}

decl_error! {
    pub enum Error for Module<T: Config> {
        /// Problem has been already registered
        ProblemAlreadyExists,
        /// Problem has not been registered yet
        ProblemNotExists,
        /// Problem has been already solved
        ProblemAlreadySolved,
        /// Proposed solution is incorrect
        WrongSolution,
        /// Not enought founds for the reward
        NotEnoughtFunds,
        /// Solution is not prime number
        NotPrimeNumber,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {

        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;


        /// Allow the owner to register problem
        #[weight = 10_000]
        fn register_problem(origin, problem: u32, reward: BalanceOf<T>) {
            let sender = ensure_signed(origin)?;

            ensure!(!Problems::<T>::contains_key(problem), Error::<T>::ProblemAlreadyExists);
            ensure!(T::Currency::can_reserve(&sender,reward), Error::<T>::NotEnoughtFunds);


            T::Currency::reserve(&sender,reward)?;
            Problems::<T>::insert(problem, (&sender,reward));

            Self::deposit_event(RawEvent::ProblemCreated(sender, problem));
        }

        /// Allow the owner to register problem
        #[weight = 10_000]
        fn solve_problem(origin, problem: u32, a:u32, b:u32) {
            let sender = ensure_signed(origin)?;

            ensure!(Problems::<T>::contains_key(problem), Error::<T>::ProblemNotExists);
            ensure!(!SolvedProblems::contains_key(problem), Error::<T>::ProblemAlreadySolved);
            ensure!(Self::is_prime(a), Error::<T>::NotPrimeNumber);
            ensure!(Self::is_prime(b), Error::<T>::NotPrimeNumber);

            // TODO check if those are prime numbers !
            ensure!(a*b == problem, Error::<T>::WrongSolution);

            let (author, reward) = Problems::<T>::get(&problem);

            let pool_reward = reward * 20.into() / 100.into();
            let solution_reward = reward - pool_reward;

            T::Currency::repatriate_reserved(&author, &sender, solution_reward, frame_support::traits::BalanceStatus::Free)?;
            T::Currency::repatriate_reserved(&author, &Self::account_id(), pool_reward, frame_support::traits::BalanceStatus::Free)?;
            SolvedProblems::insert(problem, (a,b));
            Self::deposit_event(RawEvent::ProblemSolved(sender, problem));
        }
    }
}

// initialize pallet account
const PALLET_ID: ModuleId = ModuleId(*b"__pool__");
type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

impl<T: Config> Module<T> {
    /// The account ID that holds the 10% of rewards
    pub fn account_id() -> T::AccountId {
        PALLET_ID.into_account()
    }

    fn is_prime(n: u32) -> bool {
        if n <= 1 {
            return false;
        }
        for a in 2..n {
            if n % a == 0 {
                return false; // if it is not the last statement you need to use `return`
            }
        }
        true // last value to return
    }
}
