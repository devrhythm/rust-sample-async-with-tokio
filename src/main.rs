use tokio;

// The #[tokio::main] attribute macro is used to set up the Tokio runtime
// for an asynchronous main function in Rust.\
// When you apply this macro to your main function, it automatically initializes the Tokio runtime
// and allows you to use asynchronous code within the main function
#[tokio::main]
async fn main() {
    // Note: await
    // await is used to pause the execution of an asynchronous function
    // until the awaited future is complete.
    // It allows you to write asynchronous code that looks and behaves like synchronous code
    // making it easier to read and maintain

    // Note: wnwrap()
    // unwrap() after await is a common pattern in Rust when working with asynchronous code.
    // It allows you to handle the result of an asynchronous operation
    // and deal with any potential errors
    // If the operation is successful, it returns the value; otherwise, it panics

    tokio::spawn(async {
        println!("Hello tokio 1");
    })
    .await
    .unwrap();

    tokio::spawn(async {
        println!("Hello tokio 2");
    })
    .await
    .unwrap();

    println!("main thread done");
    // output:
    // Hello tokio 1
    // Hello tokio 2
    // main thread done <- print after all tasks are done

    // Note: tokio::join!
    // macro in Rust is used to run multiple asynchronous tasks concurrently.
    // It allows you to await multiple futures simultaneously and returns a tuple of their outputs.
    // Each future is executed in parallel,
    // and the macro waits for all of them to complete before returning the results
    let task_1 = tokio::spawn(async {
        println!("Task 1: Done");
    });

    let task_2 = tokio::spawn(async {
        println!("Task 2: Done");
    });

    let _ = tokio::join!(task_1, task_2);

    println!("main thread done");
    // output: depends on which task finishes first.
    // Task 2: Done
    // Task 1: Done
    // main thread done  <- print after all tasks are done
}
