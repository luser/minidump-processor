# Version 0.9.6 (2021-12-08)

Commit: This One!

Breaking changes to fix integration issues found during deployment.

More docs.


Changes:


## minidump-stackwalk/minidump-processor

**BREAKING CHANGE**: json schema's `crashing_thread.thread_index` renamed to `crashing_thread.threads_index`

This was always supposed to be the name, we just typo'd it before publishing and didn't notice.


**BREAKING CHANGE**: minidump-stackwalk has changed its default output format from --json
to --human. Note that the --json flag was added in the previous version, so you can just
unconditionally pass --json for both versions to smooth migration.

This change was made to reflect the fact that most users of other flavours of minidump-stackwalk expect the breakpad human-based output more than mozilla's json-based output, minimizing workflow breakage. It's also just the more reasonable output for "casual" usage.









# Version 0.9.5 (2021-12-01)

Commit: 445431ce2bfe55fd85b990bb2a5c01867d2a8150

The JSON schema and minidump-stackwalk CLI are now stabilized. They are now
reasonable to rely on in production (only reason we would break them is if
we ran into a nasty bug).

This release also adds a ton of documentation! (But there can always be more...)



Changes:

## rust-minidump

Lots more documentation.

## minidump-stackwalk/minidump-processor


Breaking changes:

* Fixed symbols-paths to actually be positional (wasn't supposed to be named)
* Fixed the fact that --symbols-url accepted multiple values per instance
    * You can still pass multiple --symbols-url flags to set multiple http sources, but each one can only have one value
    * This prevents --symbols-url from accidentally greedily parsing the minidump path as one of its arguments
* Legacy truncation fields have been removed from the JSON Schema
    * `frames_truncated` removed because it was always `false`
    * `total_frames` removed because it was always the same as `frame_count`
    * Both were for a misfeature of a previous incarnation of minidump-stackwalk that we won't implement


New features:

* Cleaned up CLI help messages
* Added "--cyborg=path/to/output/json" output option (producing both --json and --human)
* Added --brief flag for shorter --human output
    * Also introduces ProcessState::print_brief
* Added dummy --json flag to hang docs off of (and to let you be explicit if you want)
* Better feedback for corrupt minidumps
* Added JSON Schema document: https://github.com/luser/rust-minidump/blob/master/minidump-processor/json-schema.md
    * JSON Schema is now stabilized








# Version 0.9.4 (2021-11-19)

Commit: [8308577df997bae72cf952ddbfaeb901a992d950](https://github.com/luser/rust-minidump/commit/8308577df997bae72cf952ddbfaeb901a992d950)

Removing derelict experiments, and one bugfix.

Changes:

## ARM Bugfix

minidump-processor's ARM stackwalker should no longer infinitely loop on misbehaving inputs.


## Removed Code

The experimental native DWARF debuginfo symbolizer has been removed from minidump-processor. This code was still technically functional, but it was using very old libraries and not being hooked into new features of minidump-processor. Not worth the maintenance burden until we have a clearer plan for it.

The private minidump-tools subcrate has been completely removed from the project. This has no affect on users using the crates published on crates.io, as it wasn't published. It was a collection of random experiments and tools that are more work to maintain than they're worth now that minidump-processor and minidump-dump work as well as they do. Also it just had some really ancient dependencies -- removing it massively reduces the amount of work needed to compile the workspace.







# Version 0.9.3 (2021-11-18)

Commit: [1e7cc1a18399e32b5589d95575447e5f159d275d](https://github.com/luser/rust-minidump/commit/1e7cc1a18399e32b5589d95575447e5f159d275d)

New features added to make symbol downloading more reliable.

Changes:

* vendored-openssl feature added to minidump-stackwalk
    * Allows you to statically link openssl (useful for docker)
* `--symbol-download-timeout-secs` flag added to minidump-stackwalk
    * Sets a timeout for downloading symbol files
    * Forces forward progress for misbehaving http response bodies
    * Default is 1000 seconds for one file

This is a breaking change for the constructor of HttpSymbolSupplier, as it now requires the timeout.








# Version 0.9.2 (2021-11-10)

Commit: [4d96a5c49a5e36cf8905cefd5ad8a5041c0d2e72](https://github.com/luser/rust-minidump/commit/4d96a5c49a5e36cf8905cefd5ad8a5041c0d2e72)

Tentative parity with mozilla/minidump-stackwalk (and all the breakpad features it uses)! 🎉

All that remains before a potential 1.0 release is testing/documenting/cleanup.


Changes:


## minidump

New features:

* GetLastError
    * MinidumpThread now has a method to retrieve the thread's GetLastError value
    * We now parse more Windows error codes

* MemoryInfo:
    * MemoryInfoListStream has been implemented (as `MinidumpMemoryInfoList`)
        * Provides metadata on the mapped memory regions like "was executable" or "was it freed"
    * LinuxMapsStream has been implemented (as `MinidumpLinuxMaps`)
        * Linux version of `MemoryInfoListStream` (using a dump of `/proc/self/maps`)
    * New `UnifiedMemoryInfoList` type
        * Takes both `MemoryInfoList` and `LinuxMaps` provides a unified memory metadata interface

* Linux Streams:
    * New Linux strings types (`LinuxOsString` and `LinuxOsStr`) to represent the fact that some values contain things like raw linux paths (and therefore may not be utf8).
    * Various simple Linux streams have minimal implementations that are exposed as a key-value pair iterator (and also just let you get the raw bytes of the dumped section).
        * LinuxCpuInfoStream (as `MinidumpLinuxCpuInfo`)
            * A dump of `/proc/cpuinfo`
        * LinuxProcStatus (as `MinidumpLinuxProcStatus`) 
            * A dump of `/proc/self/status`
        * LinuxEnviron (as `MinidumpLinuxEnviron`)
            * A dump of `/proc/self/environ`
        * LinuxLsbRelease (as `MinidumpLinuxLsbRelease`)
            * A dump of `/etc/lsb-release`
    * Because these streams are just giant bags of random info, it's hard to reasonably pick out specific values to expose. The iterator API at least makes it so you can get whatever you want easily.


Improvements:

* Contexts with XSTATE are now properly parsed.
    * (although we still ignore the XSTATE data, but previously we would have returned an error)
* minidump_dump now properly handles bad stack RVAs properly.
* MinidumpSystemInfo::csd_version now works
    * Was reading its value from the wrong array *shrug*
    * This also improves minidump processor's `os_ver` string (now at parity with breakpad)
* More docs and tests backfilled (including synth-minidump framework).
* More misbehaving logging removed
* synth-minidump has been pulled out into a separate crate so the other crates can use it for testing.


Breaking changes:

* `MinidumpThread` and `MinidumpException` now lazily parse their `context` value (and `stack` for
`MinidumpThread`).
    * This is because these values cannot be reliable parsed without access to other streams.
    * These fields have been private, in favour of accessors which require the other streams
    necessary to properly parse them.
    * `print` functionality for them (and `MinidumpThreadList`) now also takes those values.
    * For most users this won't be a big deal since you'll want all the dependent streams anyway.
* Some explicitly typed iterators have been replaced with `impl Iterator`
    * These were always supposed to be like that, this code just pre-existed the feature
    * Comes with minor efficiency win because they were internally boxed and dynamically dispatched(!) to simulate `impl Iterator`.
 * LinuxLsbRelease has had all its parsed out values removed in favour of the new iterator API. The logic that parsed out specific fields has been moved to minidump-processor.
 * LinuxLsbRelease (and some others?) now borrow the Minidump.



## minidump-stack/minidump-processor/breakpad-symbols

Thread names:

* Now can retrieve thread names from the evil_json (if this means nothing to you, don't worry about it.)


Symbol cache:

* Now writes (and reads back) an `INFO URL` line to the symbol file
    * This allows `modules[].symbol_url` in the json schema to be populated even on cache hit


Json schema:

* Now properly populates the `thread.last_error_value` field
* Now properly populates the `system_info.cpu_microcode` field (using `LinuxCpuInfoStream`)
* `system_info.os_ver` now includes the contents of `MinidumpSystemInfo::csd_version` (as intended)


Breaking changes:

* `process_minidump_with_evil` has been replaced with the more general `process_minidump_with_options`



## minidump-common

* More Windows error type definitions
* CONTEXT_HAS_XSTATE value added
* doc cleanups









# Version 0.9.1 (2021-10-27)

Commit: [15d73f888c019517411329213c2671d59335f957](https://github.com/luser/rust-minidump/commit/15d73f888c019517411329213c2671d59335f957)

Iterating closer to parity with mozilla's minidump-stackwalk!

Changes:


## minidump-stackwalk

json schema:

* "exploitability" is now `null` instead of "TODO"
* modules now have more debug stats:
    * "missing_symbols"
    * "loaded_symbols"
    * "corrupt_symbols"
    * "symbol_url"
* modules now have "filename" actually be the filename and not full path
* modules now have "cert_subject" indicating the module was code signed
* new top level field "modules_contains_cert_info" (indicating whether
  we have any known-signed modules.)

cli:
* cli has just been massively cleaned up, now has much more documentation
* --symbols-tmp is now implemented
    * Symbols that are downloaded are now downloaded to this location and
      atomically swapped into the cache, allowing multiple processes to
      share the cache safely.
* --symbols-tmp and --symbols-cache now default to using std::env::temp_dir()
  to improve portability/ergonomics
* new flags for writing output to specific files
    * --output-file
    * --log-file
* --raw-json flag is now implemented
    * feeds into the certificate info in the json schema
    * please don't use this unless you're mozilla
        * if you are mozilla please stop using this too
* logging should be a bit less noisy


## breakpad-symbols/minidump-processor

* Symbolizers now have a `stats` method for getting stats on the symbols
    * See minidump-stackwalk's new "debug stats"
* Symbolizing now has tweaked error types
    * Can now distinguish between
        * "had symbols but address had no entry" and "had no symbols"
        * this is used to refine stack scanning in the unwinder
    * Can now distinguish between "failed to load" and "failed to parse"
        * Surfaced in "corrupt_symbols" statistic
* Symbolizer now truncates PUBLIC entries if there is a FUNC record in the way
    * Reduces the rate of false-positive symbolications
* Unwinding quality has been massively improved
* Unwinders now handle STACK WIN cfi
* Unwinders now more intelligently select how hard they validate output frames
    * "better" techniques like CFI and Frame Pointers get less validation
    * This means we will happily unwind into a frame we don't have symbols for
      with CFI and Frame Pointers, which makes subsequent Scan and Frame Pointer
      unwinds more reliable (since they're starting from a more accurate position).
* Unwinders now handle ARM64 pointer auth (high bits masked off)


## rust-minidump/minidump-common/minidump-tools

* Should be largely unchanged. Any changes are incidental to refactors.


## misc

* removed some excessive logging
* fixed some panics (an overflow and over-permissive parser)





# Previous Versions

No previous versions have release notes (too early in development to worry about it).
