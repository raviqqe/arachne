#[cfg(test)]
use futures::{stream::iter, Stream, StreamExt};
#[cfg(test)]
use std::io;

#[cfg(test)]
pub fn lines_stream(string: &str) -> impl Stream<Item = Result<String, io::Error>> + '_ {
    iter(string.lines()).map(|line| Ok(line.trim().to_owned()))
}
