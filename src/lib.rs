//! Rust Practice Module

/*
Use "#[cfg(test)]" to define a test module,
the function in test module will only compile and run when user execute "cargo test" command.

Functions used by tests but not used by other modules will not receive
warnning message like "warning: function is never used: `...`".

Use "#[allow(dead_code)]" to dsiable warning of unused code.
Macro has similar problem, use "#[allow(unused_macros)]" to disable warning of unused macro.
*/
#[allow(dead_code, unused_macros)]
mod leetcode;
