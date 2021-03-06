use objc::runtime::{BOOL, YES};
use std::mem::MaybeUninit;
use std::os::raw::{c_int, c_ulonglong};

#[derive(Default)]
#[repr(C)]
pub struct Time {
    hour: c_int,
    minute: c_int,
}

#[derive(Default)]
#[repr(C)]
pub struct Schedule {
    from_time: Time,
    to_time: Time,
}

#[derive(Default)]
#[repr(C)]
pub struct InnerStatus {
    _active: BOOL,
    enabled: BOOL,
    _sun_schedule_permitted: BOOL,
    mode: c_int,
    schedule: Schedule,
    _disable_flags: c_ulonglong,
    _available: BOOL,
}

#[derive(Default)]
pub struct BlueLightStatus {
    inner: InnerStatus,
}

impl BlueLightStatus {
    pub fn c_ptr() -> MaybeUninit<InnerStatus> {
        MaybeUninit::uninit()
    }

    pub fn sched_ptr(from: (u8, u8), to: (u8, u8)) -> Schedule {
        Schedule {
            from_time: Time {
                hour: from.0 as i32,
                minute: from.1 as i32,
            },
            to_time: Time {
                hour: to.0 as i32,
                minute: to.1 as i32,
            },
        }
    }

    pub fn new(inner: InnerStatus) -> BlueLightStatus {
        BlueLightStatus { inner }
    }

    pub fn enabled(&self) -> bool {
        self.inner.enabled == YES
    }

    pub fn mode(&self) -> i32 {
        self.inner.mode as i32
    }

    pub fn from_time(&self) -> (u8, u8) {
        (
            self.inner.schedule.from_time.hour as u8,
            self.inner.schedule.from_time.minute as u8,
        )
    }

    pub fn to_time(&self) -> (u8, u8) {
        (
            self.inner.schedule.to_time.hour as u8,
            self.inner.schedule.to_time.minute as u8,
        )
    }
}
