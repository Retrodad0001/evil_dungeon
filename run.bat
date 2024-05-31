cls

cargo fmt

//run cargo test and break if failed
cargo test
if %ERRORLEVEL% NEQ 0 exit /b %ERRORLEVEL%

cargo clippy

cargo run