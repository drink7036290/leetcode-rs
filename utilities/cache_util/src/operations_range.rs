use mockall::automock;
use std::ops::RangeInclusive;

// Number of Operations: At most 2 * 10^5 calls to get and put
// reduce this number to make the tests faster
#[automock]
pub trait OperationsRangeProvider {
    fn operations_range(&self) -> RangeInclusive<usize>;
}

pub struct DefaultOperationsRangeProvider;

impl OperationsRangeProvider for DefaultOperationsRangeProvider {
    fn operations_range(&self) -> RangeInclusive<usize> {
        1..=10_000
    }
}

// A helper function that returns a MockOperationsRangeProvider with a default range pre-configured.
pub fn mock_operations_range_provider_default() -> MockOperationsRangeProvider {
    let mut mock = MockOperationsRangeProvider::new();
    // Set the default return value here, so users of this function don't have to:
    mock.expect_operations_range().return_const(1..=200);
    mock
}
