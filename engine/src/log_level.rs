pub trait LogLevel {
    const INFO: bool;
    const DEBUG: bool;
}

pub struct LogNone;

impl LogLevel for LogNone {
    const INFO: bool = false;
    const DEBUG: bool = false;
}

pub struct LogInfo;
impl LogLevel for LogInfo {
    const INFO: bool = true;
    const DEBUG: bool = false;
}

pub struct LogDebug;
impl LogLevel for LogDebug {
    const INFO: bool = true;
    const DEBUG: bool = true;
}
