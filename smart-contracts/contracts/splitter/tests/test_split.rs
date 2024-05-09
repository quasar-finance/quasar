use std::str::FromStr;

use cw_it::multi_test::MultiTestRunner;
use cw_it::{robot, OwnedTestRunner, TestRunner};
use cw_it::traits::CwItRunner;

pub const TEST_RUNNER: &str = "multi-test";


struct TestingRobot<'a>(&'a TestRunner<'a>);


#[test]
fn test_split() {
    let runner  = OwnedTestRunner::from_str(TEST_RUNNER).unwrap();

    let robot = TestingRobot(&runner.as_ref());
    
}