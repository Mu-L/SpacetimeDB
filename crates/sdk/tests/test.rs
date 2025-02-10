macro_rules! declare_tests_with_suffix {
    ($lang:ident, $suffix:literal) => {
        mod $lang {
            use spacetimedb_testing::sdk::Test;

            const MODULE: &str = concat!("sdk-test", $suffix);
            const CLIENT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/test-client");

            fn make_test(subcommand: &str) -> Test {
                Test::builder()
                    .with_name(subcommand)
                    .with_module(MODULE)
                    .with_client(CLIENT)
                    .with_language("rust")
                    .with_bindings_dir("src/module_bindings")
                    .with_compile_command("cargo build")
                    .with_run_command(format!("cargo run -- {}", subcommand))
                    .build()
            }

            #[test]
            fn insert_primitive() {
                make_test("insert_primitive").run();
            }

            #[test]
            fn subscribe_and_cancel() {
                make_test("subscribe_and_cancel").run();
            }

            #[test]
            fn subscribe_and_unsubscribe() {
                make_test("subscribe_and_unsubscribe").run();
            }

            #[test]
            fn subscription_error_smoke_test() {
                make_test("subscription_error_smoke_test").run();
            }
            #[test]
            fn delete_primitive() {
                make_test("delete_primitive").run();
            }

            #[test]
            fn update_primitive() {
                make_test("update_primitive").run();
            }

            #[test]
            fn insert_identity() {
                make_test("insert_identity").run();
            }

            #[test]
            fn insert_caller_identity() {
                make_test("insert_caller_identity").run();
            }

            #[test]
            fn delete_identity() {
                make_test("delete_identity").run();
            }

            #[test]
            fn update_identity() {
                make_test("delete_identity").run();
            }

            #[test]
            fn insert_connection_id() {
                make_test("insert_connection_id").run();
            }

            #[test]
            fn insert_caller_connection_id() {
                make_test("insert_caller_connection_id").run();
            }

            #[test]
            fn delete_connection_id() {
                make_test("delete_connection_id").run();
            }

            #[test]
            fn update_connection_id() {
                make_test("delete_connection_id").run();
            }

            #[test]
            fn insert_timestamp() {
                make_test("insert_timestamp").run();
            }

            #[test]
            fn insert_call_timestamp() {
                make_test("insert_call_timestamp").run();
            }

            #[test]
            fn on_reducer() {
                make_test("on_reducer").run();
            }

            #[test]
            fn fail_reducer() {
                make_test("fail_reducer").run();
            }

            #[test]
            fn insert_vec() {
                make_test("insert_vec").run();
            }

            #[test]
            fn insert_option_some() {
                make_test("insert_option_some").run();
            }

            #[test]
            fn insert_option_none() {
                make_test("insert_option_none").run();
            }

            #[test]
            fn insert_struct() {
                make_test("insert_struct").run();
            }

            #[test]
            fn insert_simple_enum() {
                make_test("insert_simple_enum").run();
            }

            #[test]
            fn insert_enum_with_payload() {
                make_test("insert_enum_with_payload").run();
            }

            #[test]
            fn insert_delete_large_table() {
                make_test("insert_delete_large_table").run();
            }

            #[test]
            fn insert_primitives_as_strings() {
                make_test("insert_primitives_as_strings").run();
            }

            // #[test]
            // fn resubscribe() {
            //     make_test("resubscribe").run();
            // }

            #[test]
            #[should_panic]
            fn should_fail() {
                make_test("should_fail").run();
            }

            #[test]
            fn reauth() {
                make_test("reauth_part_1").run();
                make_test("reauth_part_2").run();
            }

            #[test]
            fn reconnect_same_connection_id() {
                make_test("reconnect_same_connection_id").run();
            }

            #[test]
            fn connect_disconnect_callbacks() {
                Test::builder()
                    .with_name(concat!("connect_disconnect_callback_", stringify!($lang)))
                    .with_module(concat!("sdk-test-connect-disconnect", $suffix))
                    .with_client(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/tests/connect_disconnect_client"
                    ))
                    .with_language("rust")
                    .with_bindings_dir("src/module_bindings")
                    .with_compile_command("cargo build")
                    .with_run_command("cargo run")
                    .build()
                    .run();
            }

            #[test]
            fn caller_always_notified() {
                make_test("caller_always_notified").run();
            }

            #[test]
            fn subscribe_all_select_star() {
                make_test("subscribe_all_select_star").run();
            }

            #[test]
            fn caller_alice_receives_reducer_callback_but_not_bob() {
                make_test("caller_alice_receives_reducer_callback_but_not_bob").run();
            }
        }
    };
}

declare_tests_with_suffix!(rust, "");
// TODO: migrate csharp to snake_case table names
declare_tests_with_suffix!(csharp, "-cs");
