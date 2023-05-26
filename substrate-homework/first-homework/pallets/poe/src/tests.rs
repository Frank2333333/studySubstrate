use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};


//creat_claim：创建claim的测试用例
#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		assert_ok!(PoeModule::create_claim(RuntimeOrigin::signed(666666), claim.clone()));

		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((666666, frame_system::Pallet::<Test>::block_number()))
		);
		assert_eq!(<<Test as Config>::MaxClaimLength as Get<u32>>::get(), 10);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(666666), claim.clone());

		assert_noop!(
			PoeModule::create_claim(RuntimeOrigin::signed(666666), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

//revoke_claim：撤销claim的测试用例
#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(666666), claim.clone());

		assert_ok!(PoeModule::revoke_claim(RuntimeOrigin::signed(666666), claim.clone()));
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap(); 

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(666666), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn revoke_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(666666), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(RuntimeOrigin::signed(888888), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}


//transfer_claim：转移claim的测试用例
#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(666666), claim.clone());

		assert_ok!(PoeModule::transfer_claim(RuntimeOrigin::signed(666666), claim.clone(), 888888));

		let bounded_claim =
			BoundedVec::<u8, <Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
		assert_eq!(
			Proofs::<Test>::get(&bounded_claim),
			Some((888888, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn transfer_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(666666), claim.clone(), 888888),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn transfer_claim_failed_with_wrong_owner() {
	new_test_ext().execute_with(|| {
		let claim = BoundedVec::try_from(vec![0, 1]).unwrap();
		let _ = PoeModule::create_claim(RuntimeOrigin::signed(666666), claim.clone());

		assert_noop!(
			PoeModule::transfer_claim(RuntimeOrigin::signed(777777), claim.clone(), 888888),
			Error::<Test>::NotClaimOwner
		);
	})
}
