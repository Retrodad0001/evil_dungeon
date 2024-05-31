cls

cargo test
if %ERRORLEVEL% NEQ 0 exit /b %ERRORLEVEL%

cargo clippy -- -D warnings
if %ERRORLEVEL% NEQ 0 exit /b %ERRORLEVEL%

cargo run --release