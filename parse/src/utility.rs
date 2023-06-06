#[cfg(test)]
use async_stream::stream;
#[cfg(test)]
use futures::Stream;
#[cfg(test)]
use std::io;

// TODO Can we convert &str into Stream directly?
#[cfg(test)]
pub fn lines_stream(string: &str) -> impl Stream<Item = Result<String, io::Error>> + '_ {
    stream! {
        for line in string.lines() {
            yield Ok::<_, io::Error>(line.trim().to_owned());
        }
    }
}
