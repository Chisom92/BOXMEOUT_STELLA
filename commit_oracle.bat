@echo off
cd /d "c:\Users\User\Documents\GitHub\BOXMEOUT_STELLA"
"C:\Program Files\Git\bin\git.exe" add contracts/contracts/boxmeout/src/oracle.rs DEREGISTER_ORACLE_IMPLEMENTATION.md
"C:\Program Files\Git\bin\git.exe" commit -m "feat(oracle): implement deregister_oracle admin function"
"C:\Program Files\Git\bin\git.exe" push
pause
