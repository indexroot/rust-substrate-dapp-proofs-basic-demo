use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

use super::*;

//test标签表示是一个测试用例
#[test]
fn create_claim_work() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        assert_eq!(
            Proofs::<Test>::get(&claim),
            Some((1,frame_system::Pallet::<Test>::block_number()))
        );

    })
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );

    })

}


//作业第二题
#[test]
fn create_claim_failed_when_claim_too_large() {
    new_test_ext().execute_with(|| {
        let claim = vec![0;257];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimToolarge
        );

    })

}



#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        //作业非拥有人revoke，错误 NotClaimOwner
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner
        );

        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim),None);

    })
}


//作业转移存证
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0,1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        //非拥有人转移，错误 NotClaimOwner
        assert_noop!(
            PoeModule::transfer_accountid_claim(
                Origin::signed(2),
                 claim.clone(),
                3
            ),
            Error::<Test>::NotClaimOwner
        );

        //成功转移
        assert_ok!(PoeModule::transfer_accountid_claim(
            Origin::signed(1),
             claim.clone(),
            2
        ));

        //验证
        assert_eq!(Proofs::<Test>::get(&claim).unwrap().0,2);

    })
}









// #[test]
// fn it_works_for_default_value() {
// 	new_test_ext().execute_with(|| {
// 		// Dispatch a signed extrinsic.
// 		assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
// 		// Read pallet storage and assert an expected result.
// 		assert_eq!(TemplateModule::something(), Some(42));
// 	});
// }
//
// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(
// 			TemplateModule::cause_error(Origin::signed(1)),
// 			Error::<Test>::NoneValue
// 		);
// 	});
// }
