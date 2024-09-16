// The File struct represents a file that has been opened
// - it wraps a file descriptor
// - gives read and/or write access to the underlying file.

// All the File methods return the io::Result<T> type
// - which is an alias for Result<T, io::Error>.
// The OpenOptions struct can be used to configure how a file is opened
pub mod create;
pub mod open;
pub mod read_lines;
