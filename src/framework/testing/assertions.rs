use std::fmt::Debug;

/// Trait for making common assertions in tests
pub trait Assertions {
    /// Assert that two values are equal
    fn assert_equals<T: PartialEq + Debug>(&self, actual: T, expected: T) -> &Self;
    
    /// Assert that a value is true
    fn assert_true(&self, value: bool) -> &Self;
    
    /// Assert that a value is false
    fn assert_false(&self, value: bool) -> &Self;
    
    /// Assert that an option contains a value
    fn assert_some<T: Debug>(&self, option: Option<T>) -> &Self;
    
    /// Assert that an option is None
    fn assert_none<T>(&self, option: Option<T>) -> &Self;
    
    /// Assert that a result is Ok
    fn assert_ok<T: Debug, E>(&self, result: Result<T, E>) -> &Self;
    
    /// Assert that a result is Err
    fn assert_err<T, E: Debug>(&self, result: Result<T, E>) -> &Self;
    
    /// Assert that a collection has a specific length
    fn assert_count<T>(&self, collection: &[T], count: usize) -> &Self;
    
    /// Assert that a collection is empty
    fn assert_empty<T>(&self, collection: &[T]) -> &Self;
    
    /// Assert that a collection is not empty
    fn assert_not_empty<T>(&self, collection: &[T]) -> &Self;
    
    /// Assert that a collection contains an item
    fn assert_contains<T: PartialEq + Debug>(&self, collection: &[T], item: &T) -> &Self;
    
    /// Assert that a collection does not contain an item
    fn assert_not_contains<T: PartialEq + Debug>(&self, collection: &[T], item: &T) -> &Self;
}

/// Implementation of assertions for any type
impl<S> Assertions for S {
    fn assert_equals<T: PartialEq + Debug>(&self, actual: T, expected: T) -> &Self {
        assert_eq!(actual, expected);
        self
    }
    
    fn assert_true(&self, value: bool) -> &Self {
        assert!(value);
        self
    }
    
    fn assert_false(&self, value: bool) -> &Self {
        assert!(!value);
        self
    }
    
    fn assert_some<T: Debug>(&self, option: Option<T>) -> &Self {
        assert!(option.is_some());
        self
    }
    
    fn assert_none<T>(&self, option: Option<T>) -> &Self {
        assert!(option.is_none());
        self
    }
    
    fn assert_ok<T: Debug, E>(&self, result: Result<T, E>) -> &Self {
        assert!(result.is_ok());
        self
    }
    
    fn assert_err<T, E: Debug>(&self, result: Result<T, E>) -> &Self {
        assert!(result.is_err());
        self
    }
    
    fn assert_count<T>(&self, collection: &[T], count: usize) -> &Self {
        assert_eq!(
            collection.len(),
            count,
            "Expected collection to have {} items but got {}",
            count,
            collection.len()
        );
        self
    }
    
    fn assert_empty<T>(&self, collection: &[T]) -> &Self {
        assert!(
            collection.is_empty(),
            "Expected collection to be empty but got {} items",
            collection.len()
        );
        self
    }
    
    fn assert_not_empty<T>(&self, collection: &[T]) -> &Self {
        assert!(
            !collection.is_empty(),
            "Expected collection to not be empty"
        );
        self
    }
    
    fn assert_contains<T: PartialEq + Debug>(&self, collection: &[T], item: &T) -> &Self {
        assert!(
            collection.contains(item),
            "Expected collection to contain {:?}",
            item
        );
        self
    }
    
    fn assert_not_contains<T: PartialEq + Debug>(&self, collection: &[T], item: &T) -> &Self {
        assert!(
            !collection.contains(item),
            "Expected collection to not contain {:?}",
            item
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    struct TestStruct;
    
    #[test]
    fn test_assertions() {
        let test = TestStruct;
        
        // Test basic assertions
        test.assert_equals(2 + 2, 4)
            .assert_true(true)
            .assert_false(false);
            
        // Test option assertions
        let some_value: Option<i32> = Some(42);
        let none_value: Option<i32> = None;
        
        test.assert_some(some_value)
            .assert_none(none_value);
            
        // Test result assertions
        let ok_result: Result<i32, &str> = Ok(42);
        let err_result: Result<i32, &str> = Err("error");
        
        test.assert_ok(ok_result)
            .assert_err(err_result);
            
        // Test collection assertions
        let collection = vec![1, 2, 3];
        
        test.assert_count(&collection, 3)
            .assert_not_empty(&collection)
            .assert_contains(&collection, &2)
            .assert_not_contains(&collection, &4);
            
        let empty: Vec<i32> = vec![];
        test.assert_empty(&empty);
    }
} 