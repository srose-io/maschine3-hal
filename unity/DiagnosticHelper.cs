using UnityEngine;
using System.Runtime.InteropServices;
using System.IO;

/// <summary>
/// Diagnostic helper to troubleshoot native plugin loading issues.
/// Attach this to a GameObject and check the console for diagnostic info.
/// </summary>
public class DiagnosticHelper : MonoBehaviour
{
    void Start()
    {
        Debug.Log("=== Maschine MK3 Native Plugin Diagnostics ===");

        // Check Unity platform
        Debug.Log($"Unity Platform: {Application.platform}");
        Debug.Log($"Unity Editor: {Application.isEditor}");

        // Check expected plugin paths
        string pluginsPath = Path.Combine(Application.dataPath, "Plugins");
        Debug.Log($"Plugins Path: {pluginsPath}");
        Debug.Log($"Plugins Path Exists: {Directory.Exists(pluginsPath)}");

        string x64Path = Path.Combine(pluginsPath, "x86_64");
        Debug.Log($"x86_64 Path: {x64Path}");
        Debug.Log($"x86_64 Path Exists: {Directory.Exists(x64Path)}");

        // Check for the library file
        string[] possibleNames = new string[] {
            "libmaschine3_hal.so",
            "maschine3_hal.so",
            "maschine3_hal.dll",
            "libmaschine3_hal.dylib"
        };

        foreach (string name in possibleNames)
        {
            string fullPath = Path.Combine(x64Path, name);
            bool exists = File.Exists(fullPath);
            Debug.Log($"Checking {name}: {(exists ? "FOUND" : "NOT FOUND")}");
            if (exists)
            {
                FileInfo fi = new FileInfo(fullPath);
                Debug.Log($"  Size: {fi.Length} bytes");
                Debug.Log($"  Full path: {fullPath}");
            }
        }

        // Try to load a simple test function
        Debug.Log("\n=== Attempting to call native function ===");
        try
        {
            System.IntPtr devicePtr = mk3_new();
            if (devicePtr == System.IntPtr.Zero)
            {
                Debug.LogWarning("mk3_new() returned NULL - device not found or driver issue");
            }
            else
            {
                Debug.Log("SUCCESS! mk3_new() returned a valid pointer");
                mk3_free(devicePtr);
                Debug.Log("Successfully freed device");
            }
        }
        catch (System.DllNotFoundException e)
        {
            Debug.LogError($"DllNotFoundException: {e.Message}");
            Debug.LogError("Possible causes:");
            Debug.LogError("1. Library file not in Assets/Plugins/x86_64/");
            Debug.LogError("2. Library file has wrong name");
            Debug.LogError("3. Library missing dependencies (run 'ldd libmaschine3_hal.so' to check)");
            Debug.LogError("4. Plugin import settings incorrect in Unity Inspector");
        }
        catch (System.Exception e)
        {
            Debug.LogError($"Unexpected error: {e.GetType().Name}: {e.Message}");
        }
    }

    // Import native functions for testing
    [DllImport("maschine3_hal", CallingConvention = CallingConvention.Cdecl)]
    private static extern System.IntPtr mk3_new();

    [DllImport("maschine3_hal", CallingConvention = CallingConvention.Cdecl)]
    private static extern void mk3_free(System.IntPtr device);
}
