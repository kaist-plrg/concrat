#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )";
runProcDumpAndValidate=$(readlink -m "$DIR/../runProcDumpAndValidate.sh");
source $runProcDumpAndValidate

stressPercentage=10
procDumpType="-c"
procDumpTrigger=20
shouldDump=true

runProcDumpAndValidate $stressPercentage $procDumpType $procDumpTrigger $shouldDump "CPU"
