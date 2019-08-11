/**
 * Use "#[cfg(test)]" to define a test module,
 * the function in test module will only compile and run when user execute "cargo test" command.
 *
 * Functions used by tests but not used by other modules will not receive
 * warnning message like "warning: function is never used: `...`".
 * Use "#[cfg(dead_code)]" to dsiable warning of the unused code
 */
#[allow(dead_code)]
mod leetcode;
