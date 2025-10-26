#[no_mangle]
pub extern "C" fn run() -> i32 {
    // Simple computation that returns success code
    let result = fibonacci(10);
    if result > 0 {
        0 // Success
    } else {
        1 // Error
    }
}

#[no_mangle]
pub extern "C" fn process_data() -> i32 {
    // Simulate data processing
    let data = vec![1, 2, 3, 4, 5];
    let sum: i32 = data.iter().sum();
    
    // Return success if sum is positive
    if sum > 0 { 0 } else { 1 }
}

#[no_mangle]
pub extern "C" fn compute_fibonacci(n: i32) -> i32 {
    fibonacci(n)
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Advanced function with memory operations (for future use)
#[no_mangle]
pub extern "C" fn complex_computation() -> i32 {
    // Simulate complex computation
    let mut sum = 0;
    for i in 1..=100 {
        sum += i * i;
    }
    
    // Return 0 for success, non-zero for different outcomes
    match sum {
        338350 => 0,  // Expected result
        _ => 2,       // Unexpected result
    }
}