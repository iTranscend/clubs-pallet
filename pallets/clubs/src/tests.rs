use crate::{mock, mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use frame_benchmarking::account;
use frame_system::Account;

const SEED: u32 = 0;

#[test]
fn adds_a_new_member_to_a_club() {
	new_test_ext().execute_with(|| {
    let club = b"rotary".to_vec();

    let member: <Test as frame_system::Config>::AccountId = account("", 1, SEED);
		// Dispatch a signed extrinsic.
		assert_ok!(ClubsModule::add_member(Origin::root(), club.clone(), member.clone()));
		
    // Read pallet storage and assert an expected result.
    crate::mock::assert_last_event::<Test>(mock::Event::ClubsModule(
      crate::Event::<Test>::MemberAdded(club.clone(), member.clone())
    ));
	});
}

#[test]
fn remove_a_member_from_a_club() {
	new_test_ext().execute_with(|| {
    // Add a member to be removed
    let club = b"tennis".to_vec();
    let member: <Test as frame_system::Config>::AccountId = account("", 1, SEED);

    assert_ok!(ClubsModule::add_member(Origin::root(), club.clone(), member.clone()));

    // assert the last event
		crate::mock::assert_last_event::<Test>(mock::Event::ClubsModule(
			crate::Event::<Test>::MemberAdded(club.clone(), member.clone()),
		));

		// now remove member
		assert_ok!(ClubsModule::remove_member(Origin::root(), club.clone(), member.clone()));

		// assert the last event
		crate::mock::assert_last_event::<Test>(mock::Event::ClubsModule(
			crate::Event::<Test>::MemberRemoved(club.clone(), member.clone()),
		));
	});
}

#[test]
fn attempts_to_add_adds_an_already_existing_member_to_a_club() {
	new_test_ext().execute_with(|| {
    let club = b"rotary".to_vec();

    let member: <Test as frame_system::Config>::AccountId = account("", 1, SEED);
    let member_2: <Test as frame_system::Config>::AccountId = account("", 1, SEED);
		// Dispatch a signed extrinsic.
		assert_ok!(ClubsModule::add_member(Origin::root(), club.clone(), member.clone()));		

    // Ensure error is thrown
    assert_noop!(
			ClubsModule::add_member(Origin::root(), club.clone(), member_2.clone()),
			Error::<Test>::MemberAlreadyExistsInClub
		);
	});
}

#[test]
fn attempts_to_add_a_member_to_a_non_existent_club() {
	new_test_ext().execute_with(|| {
    let club = b"rory".to_vec();

    let member: <Test as frame_system::Config>::AccountId = account("", 1, SEED);

    // Ensure error is thrown
    assert_noop!(
			ClubsModule::add_member(Origin::root(), club.clone(), member.clone()),
			Error::<Test>::ClubDoesNotExist
		);
	});
}

#[test]
fn attempts_to_remove_a_non_existent_member_from_a_club() {
	new_test_ext().execute_with(|| {
    let club = b"rotary".to_vec();

    let member: <Test as frame_system::Config>::AccountId = account("", 1, SEED);

    // Ensure error is thrown
    assert_noop!(
			ClubsModule::remove_member(Origin::root(), club.clone(), member.clone()),
			Error::<Test>::MemberDoesNotExistInClub
		);
	});
}
