cargo fmt

cargo test
if %ERRORLEVEL% NEQ 0 exit /b %ERRORLEVEL%

cargo clippy

cargo run