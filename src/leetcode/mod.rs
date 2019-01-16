/**
 * Use "#[cfg(test)]" to define a test module,
 * the function in test module will only compile and run when user execute "cargo test" command.
 * Functions used by tests but not used by other modules will not receive
 * warnning message like "warning: function is never used: `...`".
*/

#[cfg(test)]
mod four_sum;
#[cfg(test)]
mod reverse_integer;
#[cfg(test)]
mod three_sum;
#[cfg(test)]
mod three_sum_closest;
