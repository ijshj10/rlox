pub mod chunk;
pub mod value;
pub mod vm;

#[cfg(feature = "debug_trace")]
macro_rules! debug {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

// Non-debug version
#[cfg(not(feature = "debug_trace"))]
macro_rules! debug {
    ($( $args:expr ),*) => {};
}

#[cfg(test)]
mod tests {
    use super::*;
}
