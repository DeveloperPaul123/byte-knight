/*
 * Part of the byte-knight project
 * Author: Paul Tsouchlos (DeveloperPaul123) (developer.paul.123@gmail.com)
 * Copyright (c) 2024 Paul Tsouchlos (DeveloperPaul123)
 * GNU General Public License v3.0 or later
 * https://www.gnu.org/licenses/gpl-3.0-standalone.html
 */

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
