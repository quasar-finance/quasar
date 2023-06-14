use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
struct TestCase {
    amount: u64,
    denom: String,
    txn_input: Value,
    retry_count: i64,
    retry_interval: i64,
}

#[derive(Serialize)]
struct Bond {
    bond: EmptyStruct,
}

#[derive(Serialize)]
struct EmptyStruct {}

#[no_mangle]
pub extern "C" fn generate_random_test_case(seed: u64, count: usize, min: u64, max: u64) -> *const libc::c_char {
    let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

    let mut test_cases: Vec<TestCase> = Vec::with_capacity(count);
    for _ in 0..count {
        let random_number = rng.gen_range(min..=max);
        let txn_input = Bond {
            bond: EmptyStruct {},
        };
        let json_data = serde_json::to_value(&txn_input).unwrap();
        test_cases.push(TestCase {
            amount: random_number,
            denom: String::from("ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518"),
            txn_input: json_data,
            retry_count: 10,
            retry_interval: 0,
        });
    }

    let json_data = serde_json::to_string(&test_cases).unwrap();
    let c_string = std::ffi::CString::new(json_data).unwrap();
    c_string.into_raw()
}

