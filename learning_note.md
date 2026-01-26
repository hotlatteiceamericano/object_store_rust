1. if else block has to return concrete types but no traits. It is because it needs to know the exact size at compile time. Hence the solution is to call the trait method within the if-else block.
2. axum has a custom trait IntoResponse is eye-opening. It provides an easy/elegant interface for client like me to easily map my custom errors to the expected result type
3. the From<T> trait
  1. for struct implement the `From<T>` trait, it means how this struct can be converted from T type.
  2. for my use case of converting any
4. the Into<T> trait
  1. for struct implements `Into<T>` trait, it means this struct can be converted to T type.
5. for my case, I want to have my handler to return anyhow::Result in the axum http handler. I essentially need anyhow::Error -> AppError -> axum::IntoResponse. So that first the handler function can return Result<impl IntoResponse, AppError>.
  * next: ask if axum already implement IntoResponse for std::Result
