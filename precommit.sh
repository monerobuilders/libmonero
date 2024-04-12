#!/bin/sh
echo ""
echo "STARTING: Running precommit checks"
echo ""
# First step, run copyright script, update the output as the status
printf "1 - status: running - process: running copyright" && cd scripts && python3 copyright.py && cd .. && printf "\r1 - status: success - process: running copyright\n"
# Second step, run clippy
printf "2 - status: running - process: running clippy" && cargo clippy -q --all-targets --all-features -- -D warnings && printf "\r2 - status: success - process: running clippy\n"
# Third step, run test with no output, check if process exited with 0
printf "3 - status: running - process: running test" && if cargo test &> /dev/null; then
    printf "\r3 - status: success - process: running test\n"; 
else
    printf "\r3 - status: failed  - process: running tes\n"; 
    echo ""
    echo "RESULT: FAIL - Tests failed, please check the tests with 'cargo test' command"
    echo ""
    exit 1
fi
echo ""
echo "RESULT: SUCCESS - All precommit checks passed, you can commit now!"
echo ""