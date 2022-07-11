
// Code belongs from Pentester Academy.

#include <Windows.h>
#include <stdio.h>
#include <WtsApi32.h>
#include <string>

#pragma comment(lib, "wtsapi32.lib")

std::wstring GetUserNameFromSid(PSID sid) {
	if (sid == nullptr)
		return L"";


	WCHAR name[32], domain[32];
	DWORD  lname = _countof(name), ldomain = _countof(domain);
	SID_NAME_USE use;
	if (!LookupAccountSid(nullptr, sid, name, &lname, domain, &ldomain, &use))
		return L"";

	return std::wstring(domain) + L"\\" + name;
}

int main() {


	DWORD level = 1;
	PWTS_PROCESS_INFO_EX* info;
	DWORD count;
	BOOL success = WTSEnumerateProcessesEx(WTS_CURRENT_SERVER_HANDLE, &level, WTS_ANY_SESSION, (LPWSTR*)&info, &count);
	if (!success)
		return 1;

	for (DWORD i = 0; i < count; i++) {

		PWTS_PROCESS_INFO_EX p = *info + i;
		printf("PID : %6u Thread: %3u Session: %u %ws (useername: %ws)\n",
			p->ProcessId, p->NumberOfThreads, p->SessionId, p->pProcessName,
			GetUserNameFromSid(p->pUserSid).c_str());
	}

	WTSFreeMemoryEx(WTSTypeProcessInfoLevel1, info, count);
	return 0;

}
