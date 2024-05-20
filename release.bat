cargo test
if %ERRORLEVEL% NEQ 0 exit /b %ERRORLEVEL%

cargo clippy -- -D warnings
if %ERRORLEVEL% NEQ 0 exit /b %ERRORLEVEL%

cargo run --release
//TODO: add release in the future when the project is ready to be released for windows, linux and mac