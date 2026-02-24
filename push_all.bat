@echo off
cd /d "c:\Users\User\Documents\GitHub\BOXMEOUT_STELLA"
"C:\Program Files\Git\bin\git.exe" checkout -b oracle-deregister
"C:\Program Files\Git\bin\git.exe" add -A
"C:\Program Files\Git\bin\git.exe" commit -m "feat: implement get_lp_position and deregister_oracle functions"
"C:\Program Files\Git\bin\git.exe" push -u origin oracle-deregister
pause
