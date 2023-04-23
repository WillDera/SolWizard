#[cfg(windows)]
pub const NPX: &str = "npx.cmd";

#[cfg(not(windows))]
pub const NPX: &str = "npx";

#[cfg(windows)]
pub const NPM: &str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &str = "npm";
