## Just trying to learn process injection on windows .

### Enumeration using ![Tool Help Library](https://docs.microsoft.com/en-us/windows/win32/api/_toolhelp/):

* Step 1: Enumerate the list of processes using some tool help functions from ![Tool Help Library.](https://docs.microsoft.com/en-us/windows/win32/toolhelp/about-tool-help-functions)
* Step 2: Using function ![CreateToolhelp32Snapshot](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-createtoolhelp32snapshot), this will create a snapshot of the processes running.
* Step 3: Use function ![Process32First](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-process32first) , this will enumerate all the functions captured in the snapshot. 
* Step 4: Using a user-defined function to generate a error message. 
* Step 5: Optional ( We can also use ![Thread32First](https://docs.microsoft.com/en-us/windows/win32/api/tlhelp32/nf-tlhelp32-thread32first), to enumerate all the threads, just for fun ;).
* Step 6: Run a simple do..while loop to print the PIDs, Thread IDs, and other artifacts using this process.

Demo: 

![image](https://user-images.githubusercontent.com/49472311/178348728-de4e6efe-f5c4-4d3d-9b18-42486101a8e0.png)
