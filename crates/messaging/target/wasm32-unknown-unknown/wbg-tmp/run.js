
        import {
            WasmBindgenTestContext as Context,
            __wbgtest_console_debug,
            __wbgtest_console_log,
            __wbgtest_console_info,
            __wbgtest_console_warn,
            __wbgtest_console_error,
            default as init,
        } from './wasm-bindgen-test';

        // Now that we've gotten to the point where JS is executing, update our
        // status text as at this point we should be asynchronously fetching the
        // wasm module.
        document.getElementById('output').textContent = "Loading wasm module...";

        async function main(test) {
            const wasm = await init('./wasm-bindgen-test_bg.wasm');

            const cx = new Context();
            window.on_console_debug = __wbgtest_console_debug;
            window.on_console_log = __wbgtest_console_log;
            window.on_console_info = __wbgtest_console_info;
            window.on_console_warn = __wbgtest_console_warn;
            window.on_console_error = __wbgtest_console_error;

            // Forward runtime arguments. These arguments are also arguments to the
            // `wasm-bindgen-test-runner` which forwards them to node which we
            // forward to the test harness. this is basically only used for test
            // filters for now.
            cx.args([]);

            await cx.run(test.map(s => wasm[s]));
        }

        const tests = [];
    tests.push('__wbgt_subscriber_0');
main(tests);
