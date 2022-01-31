use minidump::system_info::{Cpu, Os};

/// Information about the system that produced a `Minidump`.
pub struct SystemInfo {
    /// The operating system that produced the minidump
    pub os: Os,
    /// A string identifying the version of the operating system.
    ///
    /// This may look like "5.1.2600" or "10.4.8", if present
    pub os_version: Option<String>,
    /// A string identifying the exact build of the operating system.
    ///
    /// This may look like "Service Pack 2" or "8L2127", if present. On Windows, this is the CSD
    /// version, on Linux extended build information.
    pub os_build: Option<String>,
    /// The CPU on which the dump was produced
    pub cpu: Cpu,
    /// A string further identifying the specific CPU
    ///
    /// For example,  "GenuineIntel level 6 model 13 stepping 8", if present.
    pub cpu_info: Option<String>,
    /// The microcode version of the cpu
    pub cpu_microcode_version: Option<u64>,
    /// The number of processors in the system
    ///
    /// Will be greater than one for multi-core systems.
    pub cpu_count: usize,
}
