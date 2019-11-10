use std::cell::RefCell;
use std::fmt;
use std::str;

use std::time::{self, Duration};

pub struct Now(());

/// Returns a struct, which when formatted, renders an appropriate `Date` header
/// value.
pub fn now() -> Now {
    Now(())
}

// Gee Alex, doesn't this seem like premature optimization. Well you see there
// Billy, you're absolutely correct! If your server is *bottlenecked* on
// rendering the `Date` header, well then boy do I have news for you, you don't
// need this optimization.
//
// In all seriousness, though, a simple "hello world" benchmark which just sends
// back literally "hello world" with standard headers actually is bottlenecked
// on rendering a date into a byte buffer. Since it was at the top of a profile,
// and this was done for some competitive benchmarks, this module was written.
//
// Just to be clear, though, I was not intending on doing this because it really
// does seem kinda absurd, but it was done by someone else [1], so I blame them!
// :)
//
// [1]: https://github.com/rapidoid/rapidoid/blob/f1c55c0555007e986b5d069fe1086e6d09933f7b/rapidoid-commons/src/main/java/org/rapidoid/commons/Dates.java#L48-L66

struct LastRenderedNow {
    bytes: [u8; 128],
    amt: usize,
    next_update: time::Instant,
}

thread_local!(static LAST: RefCell<LastRenderedNow> = RefCell::new(LastRenderedNow {
    bytes: [0; 128],
    amt: 0,
    next_update: time::Instant::now(),
}));

impl fmt::Display for Now {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        LAST.with(|cache| {
            let mut cache = cache.borrow_mut();
            let now = time::Instant::now();
            if now > cache.next_update {
                cache.update(now);
            }
            f.write_str(cache.buffer())
        })
    }
}

impl LastRenderedNow {
    fn buffer(&self) -> &str {
        str::from_utf8(&self.bytes[..self.amt]).unwrap()
    }

    fn update(&mut self, last_update: time::Instant) {
        let date = chrono::Utc::now().to_rfc2822();
        let date = date.as_bytes();
        self.bytes[..date.len()].copy_from_slice(date);
        self.amt = date.len();
        self.next_update = last_update + Duration::new(1, 0);
    }
}
