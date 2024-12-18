use trpl::Html;

// An async function
async fn page_title(url: &str) -> Option<String> {
    // We need to explicitly await futures
    // Futures in Rust are lazy: they don't do anything until you ask them to with `await`

    // `get()` will get the first part of the response, which will include headers, cookies, etc
    let response = trpl::get(url).await;
    // `text()` will get the body of the response, and will wait for the entire body to be sent
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        // `map()` will execute the closure if the Option is Some()
        .map(|title_element| title_element.inner_html())

    // `await` is a postfix keyword, to allow chaining of methods:
    // let response_text = trpl::get(url).await.text().await;
}

// Rust will compile blocks marked with the `async` keyword into a unique, anonymous data type which implements the `Future` trait
// It will compile functions marked with the `async` keyword into a non-async function whose body is an async block
// Thus, writing an async fn is equivalent to writing a function which returns a future of the return type

//
// page_title()'s non async equivelent
//
// page_title() returns an object that implements the `Future` trait
// `Future` has an associated type: `Output`, which is the same type as the return type of the async version
// fn page_title(url: &str) -> impl Future<Output = Option<String>> + '_ {
//     // The body of the original function is wrapped in an `async move` block, note that this block is what is returned from the function (blocks are expressions)
//     // The async block produces a value with the type `Option<String>`, which matches the `Output` type
//     // The async block is an `async move` due to how it uses the `name` argument
//     async move {
//         let text = trpl::get(url).await.text().await;
//         Html::parse(&text)
//             .select_first("title")
//             .map(|title| title.inner_html())
//     }
// }
// The function uses a `'_` lifetime, since the function returns a future which refers to a reference (the reference from the `url` parameter)
// We need to do this to be explicit that the resulting `Future` is bound to the lifetime of the `url` reference


fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Async code needs a runtime - a Rust crate which manages the details of executing async code
    // main() can setup a runtime, but it is not a runtime itself (which is why we cannot mark it async)
    // There are many different runtimes available, depending on the use case (e.g. a web server vs embedded applications)


    trpl::run(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}


// Each await point (every place where `await` appears), represents a place where control is handed to the runtime
// Rust needs to keep track of the state involved in the async block, 
// so that the runtime can kick off some other work and then come back to it when it's ready to try advancing this one again
// This is similar to an invisible state machine:
enum PageTitleFuture<'a> {
    GetAwaitPoint {
        url: &'a str,
    },
    TextAwaitPoint {
        response: trpl::Response,
    },
}

// To execute these state machines, a runtime is needed
// This is why main() cannot be async, something else would be needed to manage the state machine for the future main() returned, 
// but main() is the starting point of the program

