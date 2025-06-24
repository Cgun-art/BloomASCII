#include <stdio.h>

int main() {
    const char* required_os = "Unknown OS";
    const char* required_version = "Unknown Version";

    // Windows
#if defined(_WIN32) || defined(_WIN64)
    required_os = "Windows";
    required_version = "Windows 10 or later";

    // Linux
#elif defined(__linux__)
    required_os = "Linux";
    required_version = "Ubuntu 20.04+, Fedora 35+, or compatible";

    // macOS
#elif defined(__APPLE__) && defined(__MACH__)
    required_os = "macOS";
    required_version = "macOS 11 Big Sur or later";

    // Android
#elif defined(__ANDROID__)
    required_os = "Android";
    required_version = "Android 10 (API level 29) or later";

    // iOS
#elif defined(__APPLE__) && !defined(__MACH__)
    required_os = "iOS";
    required_version = "iOS 13 or later";

    // Other Unix-like but not Linux
#elif defined(__unix__)
    required_os = "Non-Linux Unix";
    required_version = "POSIX-compliant Unix (e.g., Solaris, BSD)";

#else
    required_os = "Unsupported OS";
    required_version = "N/A";
#endif

    printf("Required OS: %s\n", required_os);
    printf("Required Version: %s\n", required_version);
    return 0;
}
