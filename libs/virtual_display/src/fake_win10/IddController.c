#include "../win10/IddController.h"
#include <stdio.h>
#include <stdlib.h>
#include <newdev.h>
#include <swdevice.h>
#include <strsafe.h>
#include <cfgmgr32.h>
#include <combaseapi.h>

#include "../win10/Public.h"


const GUID GUID_DEVINTERFACE_IDD_DRIVER_DEVICE = \
{ 0x781EF630, 0x72B2, 0x11d2, { 0xB8, 0x52,  0x00,  0xC0,  0x4E,  0xAF,  0x52,  0x72 } };
//{781EF630-72B2-11d2-B852-00C04EAF5272}

BOOL g_printMsg = TRUE;
char g_lastMsg[1024];
const char* g_msgHeader = "RustDeskIdd: ";

VOID WINAPI
CreationCallback(
    _In_ HSWDEVICE hSwDevice,
    _In_ HRESULT hrCreateResult,
    _In_opt_ PVOID pContext,
    _In_opt_ PCWSTR pszDeviceInstanceId
);
// https://github.com/microsoft/Windows-driver-samples/blob/9f03207ae1e8df83325f067de84494ae55ab5e97/general/DCHU/osrfx2_DCHU_base/osrfx2_DCHU_testapp/testapp.c#L88
// Not a good way for this device, I don't not why. I'm not familiar with dirver.
BOOLEAN GetDevicePath(
    _In_ LPCGUID InterfaceGuid,
    _Out_writes_(BufLen) PTCHAR DevicePath,
    _In_ size_t BufLen
);
// https://github.com/microsoft/Windows-driver-samples/blob/9f03207ae1e8df83325f067de84494ae55ab5e97/usb/umdf_fx2/exe/testapp.c#L90
// Works good to check whether device is created before.
BOOLEAN GetDevicePath2(
    _In_ LPCGUID InterfaceGuid,
    _Out_writes_(BufLen) PTCHAR DevicePath,
    _In_ size_t BufLen
);

HANDLE DeviceOpenHandle();
VOID DeviceCloseHandle(HANDLE handle);

void SetLastMsg(const char* format, ...)
{
}

const char* GetLastMsg()
{
    return g_lastMsg;
}

BOOL InstallUpdate(LPCWSTR fullInfPath, PBOOL rebootRequired)
{
    return TRUE;
}

BOOL Uninstall(LPCWSTR fullInfPath, PBOOL rebootRequired)
{
    return TRUE;
}

BOOL IsDeviceCreated(PBOOL created)
{
    return FALSE;
}

BOOL DeviceCreate(PHSWDEVICE hSwDevice)
{
    return TRUE;
}

VOID DeviceClose(HSWDEVICE hSwDevice)
{
}

BOOL MonitorPlugIn(UINT index, UINT edid, INT retries)
{
    return TRUE;
}

BOOL MonitorPlugOut(UINT index)
{
    return TRUE;
}

BOOL MonitorModesUpdate(UINT index, UINT modeCount, PMonitorMode modes)
{
    return TRUE;
}

VOID WINAPI
CreationCallback(
    _In_ HSWDEVICE hSwDevice,
    _In_ HRESULT hrCreateResult,
    _In_opt_ PVOID pContext,
    _In_opt_ PCWSTR pszDeviceInstanceId
)
{
}

BOOLEAN
GetDevicePath(
    _In_ LPCGUID InterfaceGuid,
    _Out_writes_(BufLen) PTCHAR DevicePath,
    _In_ size_t BufLen
)
{
    return TRUE;
}

BOOLEAN GetDevicePath2(
    _In_ LPCGUID InterfaceGuid,
    _Out_writes_(BufLen) PTCHAR DevicePath,
    _In_ size_t BufLen
)
{
    return TRUE;
}

// https://stackoverflow.com/questions/67164846/createfile-fails-unless-i-disable-enable-my-device
HANDLE DeviceOpenHandle()
{
    return NULL;
}

VOID DeviceCloseHandle(HANDLE handle)
{
}

VOID SetPrintErrMsg(BOOL b)
{
}
