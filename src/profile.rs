use puffin::{GlobalProfiler, NanoSecond, ScopeDetails, Stream, now_ns};

/// Similar to [`puffin::ProfilerScope`], but implements [`Send`] and can be stopped at any thread
/// at any time. Note however that the stream will be reported on an imaginary thread.
pub struct DetachedProfilerScope {
    stream: Option<Stream>,
    offset: usize,
    start: NanoSecond,
    end: Option<NanoSecond>,
}

impl DetachedProfilerScope {
    #[inline]
    pub fn new(details: ScopeDetails, data: impl AsRef<str>) -> Self {
        let scope = GlobalProfiler::lock().register_user_scopes(&[details]);
        let mut stream = puffin::Stream::default();
        let (offset, start) = stream.begin_scope(now_ns, scope[0], data.as_ref());

        Self {
            stream: Some(stream),
            offset,
            start,
            end: None,
        }
    }

    #[inline]
    pub fn end_scope(mut self, now: NanoSecond) {
        self.end = Some(now);
        drop(self);
    }
}

impl Drop for DetachedProfilerScope {
    #[inline]
    fn drop(&mut self) {
        let now = now_ns();
        let mut stream = self.stream.take().unwrap();
        stream.end_scope(self.offset, self.end.unwrap_or(now));
        GlobalProfiler::lock().report_user_scopes(
            puffin::ThreadInfo {
                start_time_ns: None,
                name: "Detached Profiler Scopes".into(),
            },
            &puffin::StreamInfo {
                stream,
                num_scopes: 1,
                depth: 1,
                range_ns: (self.start, now),
            }
            .as_stream_into_ref(),
        );
    }
}

/// Similar to [`puffin::profile_scope`], but returns a [`DetachedProfilerScope`] that implements
/// [`Send`] and can be stopped at any thread at any time. Note however that the stream will be
/// reported on an imaginary thread.
#[macro_export]
macro_rules! start_detached_scope {
    ($name:expr) => {
        $crate::start_detached_scope!($name, "")
    };
    ($name:expr, $data:expr) => {
        if puffin::are_scopes_on() {
            let details = puffin::ScopeDetails::from_scope_name($name)
                .with_function_name(puffin::clean_function_name(
                    puffin::current_function_name!(),
                ))
                .with_file(puffin::short_file_name(file!()))
                .with_line_nr(line!());
            Some($crate::DetachedProfilerScope::new(details, $data))
        } else {
            None
        }
    };
}
