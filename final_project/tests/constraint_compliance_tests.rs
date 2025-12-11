/// Tests to verify compliance with the requirement:
/// "NOT allowed to use any third party libraries like rayon, tokio, etc and NO async code ANYWHERE"
///
/// These tests ensure that:
/// 1. No external concurrency crates are imported
/// 2. No async/await syntax is used anywhere
/// 3. No tokio, rayon, or similar libraries are present
/// 4. All concurrency uses only std::thread and std::sync

#[cfg(test)]
mod no_third_party_libraries {
    /// Verify Cargo.toml has no external dependencies
    #[test]
    fn test_no_tokio_dependency() {
        let cargo_toml = include_str!("../Cargo.toml");
        assert!(!cargo_toml.contains("tokio"), 
            "VIOLATION: tokio dependency found in Cargo.toml. \
             Project must use only std library for concurrency.");
    }

    #[test]
    fn test_no_rayon_dependency() {
        let cargo_toml = include_str!("../Cargo.toml");
        assert!(!cargo_toml.contains("rayon"),
            "VIOLATION: rayon dependency found in Cargo.toml. \
             Project must use only std library for concurrency.");
    }

    #[test]
    fn test_no_async_std_dependency() {
        let cargo_toml = include_str!("../Cargo.toml");
        assert!(!cargo_toml.contains("async-std"),
            "VIOLATION: async-std dependency found in Cargo.toml. \
             Project must use only std library for concurrency.");
    }

    #[test]
    fn test_no_crossbeam_dependency() {
        let cargo_toml = include_str!("../Cargo.toml");
        assert!(!cargo_toml.contains("crossbeam"),
            "VIOLATION: crossbeam dependency found in Cargo.toml. \
             Project must use only std library for concurrency.");
    }

    #[test]
    fn test_no_parking_lot_dependency() {
        let cargo_toml = include_str!("../Cargo.toml");
        assert!(!cargo_toml.contains("parking_lot"),
            "VIOLATION: parking_lot dependency found in Cargo.toml. \
             Project must use only std library for concurrency.");
    }

    #[test]
    fn test_no_futures_dependency() {
        let cargo_toml = include_str!("../Cargo.toml");
        assert!(!cargo_toml.contains("futures"),
            "VIOLATION: futures dependency found in Cargo.toml. \
             Project must use only std library for concurrency.");
    }

    #[test]
    fn test_dependencies_section_empty() {
        let cargo_toml = include_str!("../Cargo.toml");
        // Extract dependencies section
        if let Some(deps_start) = cargo_toml.find("[dependencies]") {
            let deps_section = &cargo_toml[deps_start..];
            let deps_end = deps_section.find("[").map(|i| i + deps_start).unwrap_or(cargo_toml.len());
            let deps_content = &cargo_toml[deps_start..deps_end];
            
            // Should only have the [dependencies] header, no actual deps listed
            let lines: Vec<&str> = deps_content.lines()
                .filter(|line| !line.starts_with('[') && !line.is_empty() && !line.starts_with('#'))
                .collect();
            
            assert!(lines.is_empty(),
                "VIOLATION: External dependencies found in Cargo.toml: {:?}\n\
                 Project must have NO external dependencies, only std library.",
                lines);
        }
    }
}

#[cfg(test)]
mod no_async_code {
    #[test]
    fn test_no_async_in_lib() {
        let lib_code = include_str!("../src/lib.rs");
        assert!(!lib_code.contains("async "),
            "VIOLATION: 'async' keyword found in src/lib.rs. \
             Project must not use async/await code.");
        assert!(!lib_code.contains("await"),
            "VIOLATION: 'await' keyword found in src/lib.rs. \
             Project must not use async/await code.");
    }

    #[test]
    fn test_no_async_in_models() {
        let models_code = include_str!("../src/models.rs");
        assert!(!models_code.contains("async "),
            "VIOLATION: 'async' keyword found in src/models.rs");
        assert!(!models_code.contains("await"),
            "VIOLATION: 'await' keyword found in src/models.rs");
    }

    #[test]
    fn test_no_async_in_error() {
        let error_code = include_str!("../src/error.rs");
        assert!(!error_code.contains("async "),
            "VIOLATION: 'async' keyword found in src/error.rs");
        assert!(!error_code.contains("await"),
            "VIOLATION: 'await' keyword found in src/error.rs");
    }

    #[test]
    fn test_no_async_in_thread_pool() {
        let thread_pool_code = include_str!("../src/thread_pool.rs");
        assert!(!thread_pool_code.contains("async "),
            "VIOLATION: 'async' keyword found in src/thread_pool.rs. \
             Thread pool must use std::thread, not async.");
        assert!(!thread_pool_code.contains("await"),
            "VIOLATION: 'await' keyword found in src/thread_pool.rs");
        // Verify it uses std::thread instead
        assert!(thread_pool_code.contains("std::thread"),
            "Thread pool should use std::thread");
    }

    #[test]
    fn test_no_async_in_file_processor() {
        let processor_code = include_str!("../src/file_processor.rs");
        assert!(!processor_code.contains("async "),
            "VIOLATION: 'async' keyword found in src/file_processor.rs");
        assert!(!processor_code.contains("await"),
            "VIOLATION: 'await' keyword found in src/file_processor.rs");
    }

    #[test]
    fn test_no_async_in_progress_tracker() {
        let progress_code = include_str!("../src/progress_tracker.rs");
        assert!(!progress_code.contains("async "),
            "VIOLATION: 'async' keyword found in src/progress_tracker.rs");
        assert!(!progress_code.contains("await"),
            "VIOLATION: 'await' keyword found in src/progress_tracker.rs");
    }

    #[test]
    fn test_no_async_in_main() {
        let main_code = include_str!("../src/bin/main.rs");
        assert!(!main_code.contains("async "),
            "VIOLATION: 'async' keyword found in src/bin/main.rs");
        assert!(!main_code.contains("await"),
            "VIOLATION: 'await' keyword found in src/bin/main.rs");
    }

    #[test]
    fn test_no_async_trait() {
        let lib_code = include_str!("../src/lib.rs");
        let models_code = include_str!("../src/models.rs");
        let thread_pool_code = include_str!("../src/thread_pool.rs");
        let main_code = include_str!("../src/bin/main.rs");
        
        for (name, code) in &[
            ("lib.rs", lib_code),
            ("models.rs", models_code),
            ("thread_pool.rs", thread_pool_code),
            ("main.rs", main_code),
        ] {
            assert!(!code.contains("async trait"),
                "VIOLATION: 'async trait' found in {}", name);
        }
    }

    #[test]
    fn test_no_async_fn() {
        let main_code = include_str!("../src/bin/main.rs");
        assert!(!main_code.contains("async fn"),
            "VIOLATION: 'async fn' found in src/bin/main.rs");
    }
}

#[cfg(test)]
mod verify_std_only_usage {
    /// Verify that concurrency uses ONLY standard library
    #[test]
    fn test_uses_std_thread() {
        let thread_pool_code = include_str!("../src/thread_pool.rs");
        assert!(thread_pool_code.contains("use std::thread"),
            "Thread pool should import std::thread");
    }

    #[test]
    fn test_uses_std_sync_mutex() {
        let thread_pool_code = include_str!("../src/thread_pool.rs");
        assert!(thread_pool_code.contains("Mutex") && thread_pool_code.contains("std::sync"),
            "Should use std::sync::Mutex for thread safety");
    }

    #[test]
    fn test_uses_std_sync_condvar() {
        let thread_pool_code = include_str!("../src/thread_pool.rs");
        assert!(thread_pool_code.contains("std::sync::Condvar") || thread_pool_code.contains("Condvar"),
            "Should use std::sync::Condvar for signaling");
    }

    #[test]
    fn test_uses_std_sync_arc() {
        let thread_pool_code = include_str!("../src/thread_pool.rs");
        assert!(thread_pool_code.contains("std::sync::Arc") || thread_pool_code.contains("Arc<"),
            "Should use std::sync::Arc for shared ownership");
    }

    #[test]
    fn test_no_external_thread_spawn() {
        let main_code = include_str!("../src/bin/main.rs");
        let thread_pool_code = include_str!("../src/thread_pool.rs");
        let _file_processor_code = include_str!("../src/file_processor.rs");
        
        // Thread spawning should be done via std::thread in thread_pool
        let uses_std_thread = thread_pool_code.contains("thread::spawn") || 
                             thread_pool_code.contains("use std::thread");
        assert!(uses_std_thread,
            "Project should use std::thread for spawning threads");
        
        // No external thread libraries in main
        assert!(!main_code.contains("rayon::"),
            "Should not use rayon for threading");
        assert!(!main_code.contains("tokio::task"),
            "Should not use tokio for tasks");
    }

    #[test]
    fn test_progress_tracker_uses_std_sync() {
        let progress_code = include_str!("../src/progress_tracker.rs");
        assert!(progress_code.contains("Arc") && progress_code.contains("std::sync"),
            "Progress tracker should use std::sync::Arc");
        assert!(progress_code.contains("Mutex") && progress_code.contains("std::sync"),
            "Progress tracker should use std::sync::Mutex");
        assert!(progress_code.contains("atomic"),
            "Progress tracker should use std::sync::atomic for AtomicBool");
    }
}

#[cfg(test)]
mod constraint_verification {
    #[test]
    fn test_no_runtime_dependency() {
        let cargo_toml = include_str!("../Cargo.toml");
        assert!(!cargo_toml.contains("runtime"),
            "VIOLATION: 'runtime' (like tokio runtime) found in dependencies");
    }

    #[test]
    fn test_no_executor_dependency() {
        let cargo_toml = include_str!("../Cargo.toml");
        assert!(!cargo_toml.contains("executor"),
            "VIOLATION: External executor found in dependencies");
    }

    #[test]
    fn test_no_task_spawn_from_external() {
        let main_code = include_str!("../src/bin/main.rs");
        assert!(!main_code.contains("spawn_task"),
            "Should not use external task spawning");
        assert!(!main_code.contains("tokio::spawn"),
            "Should not use tokio::spawn");
        assert!(!main_code.contains("task::spawn"),
            "Should not use async_std task::spawn");
    }

    #[test]
    fn test_thread_pool_uses_message_passing() {
        let thread_pool_code = include_str!("../src/thread_pool.rs");
        // Verify message-based design (not work-stealing or other external patterns)
        assert!(thread_pool_code.contains("enum Message"),
            "Thread pool should use message passing pattern");
        assert!(thread_pool_code.contains("Message::"),
            "Thread pool should process Message types");
    }

    #[test]
    fn test_no_select_or_join_from_futures() {
        let main_code = include_str!("../src/bin/main.rs");
        let thread_pool_code = include_str!("../src/thread_pool.rs");
        
        assert!(!main_code.contains("futures::select"),
            "Should not use futures::select");
        assert!(!thread_pool_code.contains("join_all"),
            "Should not use futures::join_all");
    }

    #[test]
    fn test_constraint_statement_in_code() {
        let lib_code = include_str!("../src/lib.rs");
        // Verify at least one source file mentions the constraint
        let has_constraint_mention = 
            lib_code.contains("thread") || 
            lib_code.contains("std::sync") ||
            lib_code.contains("concurrency");
        
        assert!(has_constraint_mention,
            "Project should mention its use of std library concurrency");
    }

    #[test]
    fn test_no_pin_unpin_abuse() {
        let main_code = include_str!("../src/bin/main.rs");
        let thread_pool_code = include_str!("../src/thread_pool.rs");
        
        // Pin/Unpin are primarily for async code
        assert!(!main_code.contains("unsafe impl Pin"),
            "Should not need unsafe Pin implementations (suggests async code)");
        assert!(!thread_pool_code.contains("impl Unpin"),
            "Should not manually implement Unpin (suggests async pattern)");
    }

    #[test]
    fn test_no_future_trait() {
        let lib_code = include_str!("../src/lib.rs");
        let models_code = include_str!("../src/models.rs");
        let main_code = include_str!("../src/bin/main.rs");
        
        for (name, code) in &[
            ("lib.rs", lib_code),
            ("models.rs", models_code),
            ("main.rs", main_code),
        ] {
            // Future is from std::future, used in async
            assert!(!code.contains("impl Future"),
                "VIOLATION: Found 'impl Future' in {} - suggests async code", name);
            assert!(!code.contains("std::future::Future"),
                "VIOLATION: Found 'std::future::Future' in {} - suggests async code", name);
        }
    }
}

#[cfg(test)]
mod real_world_patterns {
    /// Tests for patterns that would violate the constraints
    #[test]
    fn test_no_select_macro() {
        let main_code = include_str!("../src/bin/main.rs");
        assert!(!main_code.contains("select!"),
            "VIOLATION: select! macro (from tokio/futures) found");
    }

    #[test]
    fn test_no_match_on_futures() {
        let main_code = include_str!("../src/bin/main.rs");
        let _thread_pool_code = include_str!("../src/thread_pool.rs");
        
        // This is a common pattern in async code
        let lines = main_code.lines().count();
        assert!(lines > 0, "Main should have content");
        
        // Verify no polls to futures
        assert!(!main_code.contains("poll()"),
            "Should not manually poll futures (suggests async pattern)");
    }

    #[test]
    fn test_thread_spawn_is_std_thread() {
        let main_code = include_str!("../src/bin/main.rs");
        if main_code.contains("thread::spawn") {
            // If using thread::spawn, should be std::thread
            assert!(main_code.contains("std::thread") || main_code.contains("use std::thread"),
                "thread::spawn should come from std::thread, not external crate");
        }
    }

    #[test]
    fn test_no_reactor_pattern() {
        let main_code = include_str!("../src/bin/main.rs");
        // Reactor pattern is key in async runtimes
        assert!(!main_code.contains("reactor"),
            "Should not implement or use reactor pattern (async characteristic)");
        assert!(!main_code.contains("event loop"),
            "Should not have event loop (async characteristic)\\");
    }
}

#[cfg(test)]
mod proof_of_constraint_compliance {
    /// Final comprehensive verification
    #[test]
    fn test_thread_pool_proof_no_external_deps() {
        let thread_pool_code = include_str!("../src/thread_pool.rs");
        
        // Must use these
        assert!(thread_pool_code.contains("use std::sync") && thread_pool_code.contains("Mutex"),
            "Must import from std::sync");
        assert!(thread_pool_code.contains("use std::thread"),
            "Must import from std::thread");
        
        // Must NOT use these
        assert!(!thread_pool_code.contains("rayon"),
            "rayon is third-party, cannot be used");
        assert!(!thread_pool_code.contains("tokio"),
            "tokio is third-party, cannot be used");
        assert!(!thread_pool_code.contains("crossbeam"),
            "crossbeam is third-party, cannot be used");
    }

    #[test]
    fn test_no_async_anywhere_comprehensive() {
        let files = vec![
            ("src/lib.rs", include_str!("../src/lib.rs")),
            ("src/models.rs", include_str!("../src/models.rs")),
            ("src/error.rs", include_str!("../src/error.rs")),
            ("src/thread_pool.rs", include_str!("../src/thread_pool.rs")),
            ("src/file_processor.rs", include_str!("../src/file_processor.rs")),
            ("src/progress_tracker.rs", include_str!("../src/progress_tracker.rs")),
            ("src/bin/main.rs", include_str!("../src/bin/main.rs")),
        ];
        
        for (filename, code) in files {
            // Check for async patterns
            assert!(
                !code.contains("async ") && 
                !code.contains("async\n") &&
                !code.contains("async\t"),
                "VIOLATION: Found 'async' keyword in {}", filename
            );
            
            // Check for await
            assert!(
                !code.contains(".await"),
                "VIOLATION: Found '.await' in {}", filename
            );
        }
    }

    #[test]
    fn test_source_tree_has_no_disallowed_patterns() {
        use std::fs;
        use std::path::{Path, PathBuf};

        // Recursively scan the src tree for any async or external runtime usage.
        fn collect_rs_files(dir: &Path, acc: &mut Vec<PathBuf>) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        collect_rs_files(&path, acc);
                    } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                        acc.push(path);
                    }
                }
            }
        }

        let mut files = Vec::new();
        collect_rs_files(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src"), &mut files);

        let disallowed = [
            "async ", "async\t", "async\n", ".await", "tokio", "rayon", "crossbeam",
            "async-std", "parking_lot", "futures", "runtime", "executor",
        ];

        for file in files {
            let content = fs::read_to_string(&file)
                .unwrap_or_else(|e| panic!("Failed to read {:?}: {}", file, e));
            let rel = file
                .strip_prefix(env!("CARGO_MANIFEST_DIR"))
                .unwrap_or_else(|_| file.as_path());

            for needle in disallowed {
                assert!(
                    !content.contains(needle),
                    "Disallowed pattern '{}' found in {:?}",
                    needle,
                    rel
                );
            }
        }
    }

    #[test]
    fn test_constraint_compliance_summary() {
        let cargo_toml = include_str!("../Cargo.toml");
        let thread_pool = include_str!("../src/thread_pool.rs");
        let main = include_str!("../src/bin/main.rs");
        
        // Summary of compliance:
        // 1. No external dependencies
        let no_deps = !cargo_toml.contains("tokio") &&
                      !cargo_toml.contains("rayon") &&
                      !cargo_toml.contains("async-std");
        assert!(no_deps, "COMPLIANCE FAIL: External dependencies found");
        
        // 2. Uses std::thread
        let uses_std_thread = thread_pool.contains("std::thread::") ||
                              thread_pool.contains("use std::thread");
        assert!(uses_std_thread, "COMPLIANCE FAIL: Should use std::thread");
        
        // 3. Uses std::sync (Mutex, Arc, Condvar.)
        let uses_std_sync = thread_pool.contains("use std::sync") && 
                           (thread_pool.contains("Mutex") ||
                            thread_pool.contains("Arc") ||
                            thread_pool.contains("Condvar"));
        assert!(uses_std_sync, "COMPLIANCE FAIL: Should use std::sync");
        
        // 4. No async code
        let no_async = !main.contains("async ") &&
                      !thread_pool.contains("async ");
        assert!(no_async, "COMPLIANCE FAIL: Found async code");
        
        // All checks passed
        println!("COMPLIANCE VERIFIED: No third-party libraries, no async code");
        println!("Uses ONLY std::thread and std::sync");
        println!("Custom thread pool implementation");
    }
}
