// When everything is fine

// It is useful to report on the application’s progress even when everything is fine. Try to be informative and concise in these messages. Don’t use overly technical terms in the logs. Remember: the application is not crashing so there’s no reason for users to look up errors.

// Most importantly, be consistent in the style of communication. Use the same prefixes and sentence structure to make the logs easily skimmable.

// Try to let your application output tell a story about what it’s doing and how it impacts the user. This can involve showing a timeline of steps involved or even a progress bar and indicator for long-running actions. The user should at no point get the feeling that the application is doing something mysterious that they cannot follow.

// When it’s hard to tell what’s going on

// When communicating non-nominal state it’s important to be consistent. A heavily logging application that doesn’t follow strict logging levels provides the same amount, or even less information than a non-logging application.

// Because of this, it’s important to define the severity of events and messages that are related to it; then use consistent log levels for them. This way users can select the amount of logging themselves via --verbose flags or environment variables (like RUST_LOG).

// The commonly used log crate defines the following levels (ordered by increasing severity):

// * trace
// * debug
// * info
// * warning
// * error

// It’s a good idea to think of info as the default log level. Use it for, well, informative output. (Some applications that lean towards a more quiet output style might only show warnings and errors by default.)

// Additionally, it’s always a good idea to use similar prefixes and sentence structure across log messages, making it easy to use a tool like grep to filter for them. A message should provide enough context by itself to be useful in a filtered log while not being too verbose at the same time.

// Example log statements

// error: could not find `Cargo.toml` in `/home/you/project/`

// => Downloading repository index
// => Downloading packages...

// [1/7] Adding WASM target...
// [2/7] Compiling to WASM...
// [3/7] Creating a pkg directory...
// [4/7] Writing a package.json...
// > [WARN]: Field `description` is missing from Cargo.toml. It is not necessary, but recommended
// > [WARN]: Field `repository` is missing from Cargo.toml. It is not necessary, but recommended
// > [WARN]: Field `license` is missing from Cargo.toml. It is not necessary, but recommended
// [5/7] Copying over your README...
// > [WARN]: origin crate has no README
// [6/7] Installing WASM-bindgen...
// > [INFO]: wasm-bindgen already installed
// [7/7] Running WASM-bindgen...
// Done in 1 second
