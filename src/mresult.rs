#![allow(unused)]

#[derive(Debug, PartialEq)]
enum MResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> MResult<T, E> {
    fn ok(value: T) -> Self {
        MResult::Ok(value)
    }
    // Function to create an Err variant
    fn err(error: E) -> Self {
        MResult::Err(error)
    }

    // Method to check if it's an Ok variant
    fn is_ok(&self) -> bool {
        match self {
            MResult::Ok(value) => true,
            MResult::Err(error) => false,
        }
    }

    // Method to check if it's an Err variant
    fn is_err(&self) -> bool {
        match self {
            MResult::Ok(value) => false,
            MResult::Err(error) => true,
        }
    }

    // Method to unwrap the Ok value, panics if it's an Err
    fn unwrap(self) -> T {
        match self {
            MResult::Ok(value) => value,
            MResult::Err(error) => panic!("Couldn't unwrap value, because it is an error"),
        }
    }

    // Method to unwrap the Err value, panics if it's an Ok
    fn unwrap_err(self) -> E {
        match self {
            MResult::Ok(value) => panic!("Couldn't unwrap error, because it is an Ok value"),
            MResult::Err(error) => error,
        }
    }
}

// Add unit tests below
#[cfg (test)]
mod tests {
    use crate::mresult::MResult;

    #[test]
    fn test_ok() {
        let result: MResult<u8, &str> = MResult::ok(10);
        assert_eq!(result, MResult::Ok(10));
    }

    #[test]
    fn test_err() {
        let result: MResult<u8, &str> = MResult::err("Error");
        assert_eq!(result, MResult::Err("Error"));
    }

    #[test]
    fn test_is_ok_true() {
        let result: MResult<u8, &str> = MResult::Ok(10);
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_ok_false() {
        let result: MResult<u8, &str> = MResult::Err("error");
        assert_eq!(result.is_ok(), false);
    }

    #[test]
    fn test_is_err_true() {
        let result: MResult<u8, &str> = MResult::Err("error");
        assert!(result.is_err())
    }

    #[test]
    fn test_is_err_false() {
        let result: MResult<u8, &str> = MResult::Ok(10);
        assert_eq!(result.is_err(), false);
    }

    #[test]
    fn test_unwrap() {
        let result: MResult<u8, &str> = MResult::Ok(18);
        assert_eq!(result.unwrap(), 18);
    }

    #[test]
    #[should_panic(expected = "Couldn't unwrap value, because it is an error")]
    fn test_unwrap_panic() {
        let result: MResult<u8, &str> = MResult::Err("error");
        result.unwrap();
    }

    #[test]
    fn test_unwrap_error() {
        let result: MResult<u8, &str> = MResult::Err("error");
        assert_eq!(result.unwrap_err(), "error");
    }

    #[test]
    #[should_panic(expected = "Couldn't unwrap error, because it is an Ok value")]
    fn test_unwrap_error_panic() {
        let result: MResult<u8, &str> = MResult::Ok(23);
        result.unwrap_err();
    }
}