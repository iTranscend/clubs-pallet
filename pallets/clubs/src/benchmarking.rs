//! Benchmarking setup for pallet-clubs
use super::*;

#[allow(unused)]
use crate::Pallet;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}
benchmarks! {
	add_member {
    let caller: T::AccountId = whitelisted_caller();

    let club_1 = b"mxc1".to_vec();
    let club_2 = b"mxc2".to_vec();

    let a_member: T::AccountId = account("account", 0, SEED);
		let members = vec![a_member];

    Clubs::<T>::insert(club_1.clone(), members.clone());
    Clubs::<T>::insert(club_2.clone(), members);
	}: _(RawOrigin::Root, club_1.clone(), caller.clone())
	verify {
    assert_last_event::<T>(Event::MemberAdded(club_1.to_vec(), caller.clone()).into());
	}

  remove_member {
    let caller: T::AccountId = whitelisted_caller();

    let club_1 = b"mxc1".to_vec();
    let club_2 = b"mxc2".to_vec();

    let a_member: T::AccountId = account("account", 0, SEED);
		let members = vec![a_member, caller.clone()];

    Clubs::<T>::insert(club_1.clone(), members.clone());
    Clubs::<T>::insert(club_2.clone(), members);

	}: _(RawOrigin::Root, club_1.clone(), caller.clone())
	verify {
    assert_last_event::<T>(Event::MemberRemoved(club_1.to_vec(), caller.clone()).into());
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}