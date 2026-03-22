use crate::core::TuioTime;

/// Base container with attributes all tuio entities share.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Container {
    pub(crate) start_time: TuioTime,
    pub(crate) current_time: TuioTime,
    pub(crate) session_id: i32,
}

impl Container {
    pub(crate) fn new(start_time: &TuioTime, session_id: i32) -> Self {
        Self {
            start_time: *start_time,
            current_time: *start_time,
            session_id,
        }
    }

    pub(crate) fn update(&mut self, time: &TuioTime) {
        self.current_time = *time;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── new ──────────────────────────────────────────────────────────────────

    #[test]
    fn new_sets_session_id() {
        let t = TuioTime::new(10, 0);
        let c = Container::new(&t, 42);
        assert_eq!(c.session_id, 42);
    }

    #[test]
    fn new_sets_start_time_equal_to_provided_time() {
        let t = TuioTime::new(5, 123_456);
        let c = Container::new(&t, 1);
        assert_eq!(c.start_time, t);
    }

    #[test]
    fn new_sets_current_time_equal_to_start_time() {
        let t = TuioTime::new(5, 123_456);
        let c = Container::new(&t, 1);
        // On creation, current_time must equal start_time exactly.
        assert_eq!(c.current_time, c.start_time);
    }

    #[test]
    fn new_current_time_matches_provided_time() {
        let t = TuioTime::new(99, 500_000);
        let c = Container::new(&t, 7);
        assert_eq!(c.current_time, t);
    }

    #[test]
    fn new_session_id_zero() {
        let t = TuioTime::new(0, 0);
        let c = Container::new(&t, 0);
        assert_eq!(c.session_id, 0);
    }

    #[test]
    fn new_negative_session_id() {
        let t = TuioTime::new(0, 0);
        let c = Container::new(&t, -1);
        assert_eq!(c.session_id, -1);
    }

    // ── update ───────────────────────────────────────────────────────────────

    #[test]
    fn update_changes_current_time() {
        let start = TuioTime::new(1, 0);
        let mut c = Container::new(&start, 10);
        let new_time = TuioTime::new(2, 500_000);
        c.update(&new_time);
        assert_eq!(c.current_time, new_time);
    }

    #[test]
    fn update_does_not_change_start_time() {
        let start = TuioTime::new(1, 0);
        let mut c = Container::new(&start, 10);
        let new_time = TuioTime::new(100, 0);
        c.update(&new_time);
        // start_time must remain the original value.
        assert_eq!(c.start_time, start);
    }

    #[test]
    fn update_does_not_change_session_id() {
        let start = TuioTime::new(0, 0);
        let mut c = Container::new(&start, 55);
        let new_time = TuioTime::new(1, 0);
        c.update(&new_time);
        assert_eq!(c.session_id, 55);
    }

    #[test]
    fn update_multiple_times_keeps_latest_current_time() {
        let start = TuioTime::new(0, 0);
        let mut c = Container::new(&start, 1);

        let t1 = TuioTime::new(1, 0);
        let t2 = TuioTime::new(2, 0);
        let t3 = TuioTime::new(3, 999_999);

        c.update(&t1);
        assert_eq!(c.current_time, t1);

        c.update(&t2);
        assert_eq!(c.current_time, t2);

        c.update(&t3);
        assert_eq!(c.current_time, t3);

        // start_time never changes across multiple updates.
        assert_eq!(c.start_time, start);
    }

    #[test]
    fn update_with_same_time_is_idempotent() {
        let start = TuioTime::new(5, 100);
        let mut c = Container::new(&start, 3);
        c.update(&start);
        assert_eq!(c.current_time, start);
        assert_eq!(c.start_time, start);
    }
}
